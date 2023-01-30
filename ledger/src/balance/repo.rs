use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};
use tracing::instrument;
use uuid::Uuid;

use std::collections::HashMap;

use super::entity::*;
use crate::{error::*, primitives::*};

#[derive(Debug, Clone)]
pub struct Balances {
    pool: PgPool,
}

impl Balances {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    #[instrument(name = "sqlx_ledger.balances.find", skip(self))]
    pub async fn find(
        &self,
        journal_id: JournalId,
        account_id: AccountId,
        currency: Currency,
    ) -> Result<Option<AccountBalance>, SqlxLedgerError> {
        let record = sqlx::query!(
            r#"SELECT
                 a.normal_balance_type as "normal_balance_type: DebitOrCredit", b.journal_id, b.account_id, entry_id, b.currency,
                 settled_dr_balance, settled_cr_balance, settled_entry_id, settled_modified_at,
                 pending_dr_balance, pending_cr_balance, pending_entry_id, pending_modified_at,
                 encumbered_dr_balance, encumbered_cr_balance, encumbered_entry_id, encumbered_modified_at,
                 version, modified_at, created_at
               FROM sqlx_ledger_balances b
               JOIN ( SELECT id, normal_balance_type FROM sqlx_ledger_accounts WHERE id = $2 LIMIT 1 ) a
                 ON a.id = b.account_id
               WHERE journal_id = $1 AND account_id = $2 AND currency = $3
               ORDER BY version DESC LIMIT 1"#,
            Uuid::from(journal_id),
            Uuid::from(account_id),
            currency.code()
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record.map(|record| AccountBalance {
            balance_type: record.normal_balance_type,
            details: BalanceDetails {
                journal_id,
                account_id,
                entry_id: EntryId::from(record.entry_id),
                currency,
                settled_dr_balance: record.settled_dr_balance,
                settled_cr_balance: record.settled_cr_balance,
                settled_entry_id: EntryId::from(record.settled_entry_id),
                settled_modified_at: record.settled_modified_at,
                pending_dr_balance: record.pending_dr_balance,
                pending_cr_balance: record.pending_cr_balance,
                pending_entry_id: EntryId::from(record.pending_entry_id),
                pending_modified_at: record.pending_modified_at,
                encumbered_dr_balance: record.encumbered_dr_balance,
                encumbered_cr_balance: record.encumbered_cr_balance,
                encumbered_entry_id: EntryId::from(record.encumbered_entry_id),
                encumbered_modified_at: record.encumbered_modified_at,
                version: record.version,
                modified_at: record.modified_at,
                created_at: record.created_at,
            },
        }))
    }
    pub(crate) async fn find_for_update<'a>(
        &self,
        journal_id: JournalId,
        ids: Vec<(AccountId, &Currency)>,
        tx: &mut Transaction<'a, Postgres>,
    ) -> Result<HashMap<AccountId, BalanceDetails>, SqlxLedgerError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"SELECT
              b.journal_id, b.account_id, entry_id, b.currency,
              settled_dr_balance, settled_cr_balance, settled_entry_id, settled_modified_at,
              pending_dr_balance, pending_cr_balance, pending_entry_id, pending_modified_at,
              encumbered_dr_balance, encumbered_cr_balance, encumbered_entry_id, encumbered_modified_at,
              c.version, modified_at, created_at
                FROM sqlx_ledger_balances b JOIN (
                    SELECT * FROM sqlx_ledger_current_balances WHERE journal_id = "#,
        );
        query_builder.push_bind(Uuid::from(journal_id));
        query_builder.push(r#" AND (account_id, currency) IN"#);
        query_builder.push_tuples(ids, |mut builder, (id, currency)| {
            builder.push_bind(Uuid::from(id));
            builder.push_bind(currency.code());
        });
        query_builder.push(
            r#"FOR UPDATE ) c ON
                b.journal_id = c.journal_id AND b.account_id = c.account_id AND b.currency = c.currency AND b.version = c.version"#,
        );

        let query = query_builder.build();
        let records = query.fetch_all(&mut *tx).await?;
        let mut ret = HashMap::new();
        for r in records {
            let account_id = AccountId::from(r.get::<Uuid, _>("account_id"));
            ret.insert(
                account_id,
                BalanceDetails {
                    account_id,
                    journal_id: JournalId::from(r.get::<Uuid, _>("journal_id")),
                    entry_id: EntryId::from(r.get::<Uuid, _>("entry_id")),
                    currency: r.get::<&str, _>("currency").parse()?,
                    settled_dr_balance: r.get("settled_dr_balance"),
                    settled_cr_balance: r.get("settled_cr_balance"),
                    settled_entry_id: EntryId::from(r.get::<Uuid, _>("settled_entry_id")),
                    settled_modified_at: r.get("settled_modified_at"),
                    pending_dr_balance: r.get("pending_dr_balance"),
                    pending_cr_balance: r.get("pending_cr_balance"),
                    pending_entry_id: EntryId::from(r.get::<Uuid, _>("pending_entry_id")),
                    pending_modified_at: r.get("pending_modified_at"),
                    encumbered_dr_balance: r.get("encumbered_dr_balance"),
                    encumbered_cr_balance: r.get("encumbered_cr_balance"),
                    encumbered_entry_id: EntryId::from(r.get::<Uuid, _>("encumbered_entry_id")),
                    encumbered_modified_at: r.get("encumbered_modified_at"),
                    version: r.get("version"),
                    modified_at: r.get("modified_at"),
                    created_at: r.get("created_at"),
                },
            );
        }
        Ok(ret)
    }

    pub(crate) async fn update_balances<'a>(
        &self,
        journal_id: JournalId,
        new_balances: Vec<BalanceDetails>,
        tx: &mut Transaction<'a, Postgres>,
    ) -> Result<(), SqlxLedgerError> {
        let mut latest_versions = HashMap::new();
        let mut previous_versions = HashMap::new();
        for BalanceDetails {
            account_id,
            version,
            currency,
            ..
        } in new_balances.iter()
        {
            latest_versions.insert((account_id, currency), version);
            if previous_versions.contains_key(&(account_id, currency)) {
                continue;
            }
            previous_versions.insert((account_id, currency), version - 1);
        }
        let expected_accounts_effected = latest_versions.len();
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"INSERT INTO sqlx_ledger_current_balances
                  (journal_id, account_id, currency, version)"#,
        );
        let mut any_new = false;
        query_builder.push_values(
            previous_versions.iter().filter(|(_, v)| **v == 0),
            |mut builder, ((account_id, currency), version)| {
                any_new = true;
                builder.push_bind(Uuid::from(journal_id));
                builder.push_bind(Uuid::from(**account_id));
                builder.push_bind(currency.code());
                builder.push_bind(version);
            },
        );
        if any_new {
            query_builder.build().execute(&mut *tx).await?;
        }
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(r#"UPDATE sqlx_ledger_current_balances SET version = CASE"#);
        let mut bind_numbers = HashMap::new();
        let mut next_bind_number = 1;
        for ((account_id, currency), version) in latest_versions {
            bind_numbers.insert((account_id, currency), next_bind_number);
            next_bind_number += 3;
            query_builder.push(" WHEN account_id = ");
            query_builder.push_bind(Uuid::from(*account_id));
            query_builder.push(" AND currency = ");
            query_builder.push_bind(currency.code());
            query_builder.push(" THEN ");
            query_builder.push_bind(version);
        }
        query_builder.push(" END WHERE journal_id = ");
        query_builder.push_bind(Uuid::from(journal_id));
        query_builder.push(" AND (account_id, currency, version) IN");
        query_builder.push_tuples(
            previous_versions,
            |mut builder, ((account_id, currency), version)| {
                let n = bind_numbers.remove(&(account_id, currency)).unwrap();
                builder.push(format!("${}, ${}", n, n + 1));
                builder.push_bind(version);
            },
        );
        let result = query_builder.build().execute(&mut *tx).await?;
        if result.rows_affected() != (expected_accounts_effected as u64) {
            return Err(SqlxLedgerError::OptimisticLockingError);
        }

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"INSERT INTO sqlx_ledger_balances (
                 journal_id, account_id, entry_id, currency,
                 settled_dr_balance, settled_cr_balance, settled_entry_id, settled_modified_at,
                 pending_dr_balance, pending_cr_balance, pending_entry_id, pending_modified_at,
                 encumbered_dr_balance, encumbered_cr_balance, encumbered_entry_id, encumbered_modified_at,
                 version, modified_at, created_at)
            "#,
        );
        query_builder.push_values(new_balances, |mut builder, b| {
            builder.push_bind(Uuid::from(b.journal_id));
            builder.push_bind(Uuid::from(b.account_id));
            builder.push_bind(Uuid::from(b.entry_id));
            builder.push_bind(b.currency.code());
            builder.push_bind(b.settled_dr_balance);
            builder.push_bind(b.settled_cr_balance);
            builder.push_bind(Uuid::from(b.settled_entry_id));
            builder.push_bind(b.settled_modified_at);
            builder.push_bind(b.pending_dr_balance);
            builder.push_bind(b.pending_cr_balance);
            builder.push_bind(Uuid::from(b.pending_entry_id));
            builder.push_bind(b.pending_modified_at);
            builder.push_bind(b.encumbered_dr_balance);
            builder.push_bind(b.encumbered_cr_balance);
            builder.push_bind(Uuid::from(b.encumbered_entry_id));
            builder.push_bind(b.encumbered_modified_at);
            builder.push_bind(b.version);
            builder.push_bind(b.modified_at);
            builder.push_bind(b.created_at);
        });
        query_builder.build().execute(&mut *tx).await?;
        Ok(())
    }
}
