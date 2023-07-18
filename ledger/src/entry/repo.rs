use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};
use tracing::instrument;
use uuid::Uuid;

use std::{collections::HashMap, str::FromStr};

use super::entity::*;
use crate::{error::*, primitives::*};

/// Repository for working with `Entry` (Debit/Credit) entities.
#[derive(Debug, Clone)]
pub struct Entries {
    pool: PgPool,
}

#[derive(Debug)]
pub(crate) struct StagedEntry {
    pub(crate) account_id: AccountId,
    pub(crate) entry_id: EntryId,
    pub(crate) units: Decimal,
    pub(crate) currency: Currency,
    pub(crate) direction: DebitOrCredit,
    pub(crate) layer: Layer,
    pub(crate) created_at: DateTime<Utc>,
}

impl Entries {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    #[instrument(
        level = "trace",
        name = "sqlx_ledger.entries.create_all",
        skip(self, tx)
    )]
    pub(crate) async fn create_all<'a>(
        &self,
        journal_id: JournalId,
        transaction_id: TransactionId,
        entries: Vec<NewEntry>,
        tx: &mut Transaction<'a, Postgres>,
    ) -> Result<Vec<StagedEntry>, SqlxLedgerError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"WITH new_entries as (
                 INSERT INTO sqlx_ledger_entries
                  (id, transaction_id, journal_id, entry_type, layer,
                   units, currency, direction, description, sequence, account_id)"#,
        );
        let mut partial_ret = HashMap::new();
        let mut sequence = 1;
        query_builder.push_values(
            entries,
            |mut builder,
             NewEntry {
                 account_id,
                 entry_type,
                 layer,
                 units,
                 currency,
                 direction,
                 description,
             }: NewEntry| {
                builder.push("gen_random_uuid()");
                builder.push_bind(transaction_id);
                builder.push_bind(journal_id);
                builder.push_bind(entry_type);
                builder.push_bind(layer);
                builder.push_bind(units);
                builder.push_bind(currency.code());
                builder.push_bind(direction);
                builder.push_bind(description);
                builder.push_bind(sequence);
                builder.push("(SELECT id FROM sqlx_ledger_accounts WHERE id = ");
                builder.push_bind_unseparated(account_id);
                builder.push_unseparated(")");
                partial_ret.insert(sequence, (account_id, units, currency, layer, direction));
                sequence += 1;
            },
        );
        query_builder.push(
            "RETURNING id, sequence, created_at ) SELECT * FROM new_entries ORDER BY sequence",
        );
        let query = query_builder.build();
        let records = query.fetch_all(&mut **tx).await?;

        let mut ret = Vec::new();
        sequence = 1;
        for r in records {
            let entry_id: Uuid = r.get("id");
            let created_at = r.get("created_at");
            let (account_id, units, currency, layer, direction) =
                partial_ret.remove(&sequence).expect("sequence not found");
            ret.push(StagedEntry {
                entry_id: entry_id.into(),
                account_id,
                units,
                currency,
                layer,
                direction,
                created_at,
            });
            sequence += 1;
        }

        Ok(ret)
    }

    pub async fn list_by_transaction_ids(
        &self,
        tx_ids: impl IntoIterator<Item = impl std::borrow::Borrow<TransactionId>>,
    ) -> Result<HashMap<TransactionId, Vec<Entry>>, SqlxLedgerError> {
        let tx_ids: Vec<Uuid> = tx_ids
            .into_iter()
            .map(|id| Uuid::from(id.borrow()))
            .collect();
        let records = sqlx::query!(
            r#"SELECT id, version, transaction_id, account_id, journal_id, entry_type, layer as "layer: Layer", units, currency, direction as "direction: DebitOrCredit", sequence, description, created_at, modified_at
            FROM sqlx_ledger_entries
            WHERE transaction_id = ANY($1) ORDER BY transaction_id ASC, sequence ASC, version DESC"#,
            &tx_ids[..]
        ).fetch_all(&self.pool).await?;

        let mut transactions: HashMap<TransactionId, Vec<Entry>> = HashMap::new();

        let mut current_tx_id = TransactionId::new();
        let mut last_sequence = 0;
        for row in records {
            let transaction_id = TransactionId::from(row.transaction_id);
            // Skip old entry versions (description is mutable)
            if last_sequence == row.sequence && transaction_id == current_tx_id {
                continue;
            }
            current_tx_id = transaction_id;
            last_sequence = row.sequence;

            let entry = transactions.entry(transaction_id).or_default();

            entry.push(Entry {
                id: EntryId::from(row.id),
                transaction_id,
                version: row.version as u32,
                account_id: AccountId::from(row.account_id),
                journal_id: JournalId::from(row.journal_id),
                entry_type: row.entry_type,
                layer: row.layer,
                units: row.units,
                currency: Currency::from_str(row.currency.as_str())
                    .expect("Couldn't convert currency"),
                direction: row.direction,
                sequence: row.sequence as u32,
                description: row.description,
                created_at: row.created_at,
                modified_at: row.modified_at,
            })
        }

        Ok(transactions)
    }
}
