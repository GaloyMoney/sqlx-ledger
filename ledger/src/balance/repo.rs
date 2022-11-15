use sqlx::{PgPool, Postgres, QueryBuilder, Row, Transaction};
use uuid::Uuid;

use std::collections::HashMap;

use super::entity::*;
use crate::{error::*, primitives::*};

pub struct Balances {
    pool: PgPool,
}

impl Balances {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub(crate) async fn find_for_update<'a>(
        &self,
        ids: Vec<AccountId>,
        tx: &mut Transaction<'a, Postgres>,
    ) -> Result<HashMap<AccountId, Balance>, SqlxLedgerError> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"SELECT
              journal_id, b.account_id, entry_id, b.currency,
              settled_dr_balance, settled_cr_balance, settled_entry_id, settled_modified_at,
              pending_dr_balance, pending_cr_balance, pending_entry_id, pending_modified_at,
              encumbered_dr_balance, encumbered_cr_balance, encumbered_entry_id, encumbered_modified_at,
              c.version AS version, created_at
                FROM balances b JOIN current_balances c 
                  ON b.account_id = c.account_id AND b.version = c.version AND b.currency = c.currency
                WHERE b.account_id IN"#,
        );
        query_builder.push_tuples(ids, |mut builder, id| {
            builder.push_bind(Uuid::from(id));
        });
        query_builder.push("FOR UPDATE OF c");

        let query = query_builder.build();
        let records = query.fetch_all(&mut *tx).await?;
        let mut ret = HashMap::new();
        for r in records {
            let account_id = AccountId::from(r.get::<Uuid, _>("account_id"));
            ret.insert(
                account_id,
                Balance {
                    account_id,
                    journal_id: JournalId::from(r.get::<Uuid, _>("journal_id")),
                    entry_id: EntryId::from(r.get::<Uuid, _>("entry_id")),
                    currency: r.get("currency"),
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
        new_balances: Vec<Balance>,
        tx: &mut Transaction<'a, Postgres>,
    ) -> Result<(), SqlxLedgerError> {
        let mut latest_versions = HashMap::new();
        let mut previous_versions = HashMap::new();
        for Balance {
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
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"INSERT INTO current_balances
                  (account_id, currency, version)"#,
        );
        let mut any_new = false;
        query_builder.push_values(
            previous_versions.iter(),
            |mut builder, ((account_id, currency), version)| {
                if version == &0 {
                    any_new = true;
                    builder.push_bind(Uuid::from(**account_id));
                    builder.push_bind(currency);
                    builder.push_bind(version);
                }
            },
        );
        if any_new {
            query_builder.build().execute(&mut *tx).await?;
        }
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new(r#"UPDATE current_balances SET version = CASE"#);
        let mut bind_numbers = HashMap::new();
        let mut next_bind_number = 1;
        for ((account_id, currency), version) in latest_versions {
            bind_numbers.insert((account_id, currency), next_bind_number);
            next_bind_number += 3;
            query_builder.push(" WHEN account_id = ");
            query_builder.push_bind(Uuid::from(*account_id));
            query_builder.push(" AND currency = ");
            query_builder.push_bind(currency);
            query_builder.push(" THEN ");
            query_builder.push_bind(version);
        }
        query_builder.push(" END WHERE (account_id, currency, version) IN");
        query_builder.push_tuples(
            previous_versions,
            |mut builder, ((account_id, currency), version)| {
                let n = bind_numbers.remove(&(account_id, currency)).unwrap();
                builder.push(format!("${}, ${}", n, n + 1));
                builder.push_bind(version);
            },
        );
        query_builder.build().execute(&mut *tx).await?;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"INSERT INTO balances (
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
            builder.push_bind(b.currency);
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
