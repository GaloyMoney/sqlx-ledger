use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use super::entity::*;
use crate::{error::*, primitives::*};

pub struct Entries {
    pool: PgPool,
}

impl Entries {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub(crate) async fn create_all<'a>(
        &self,
        journal_id: JournalId,
        transaction_id: TransactionId,
        entries: Vec<NewEntry>,
        tx: Transaction<'a, Postgres>,
    ) -> Result<Transaction<'a, Postgres>, SqlxLedgerError> {
        let mut tx = self.pool.begin().await?;
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"INSERT INTO entries
                (id, version, transaction_id, journal_id, entry_type, layer,
                 units, currency, direction, description, sequence, account_id)"#,
        );
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
                builder.push("1");
                builder.push_bind(Uuid::from(transaction_id));
                builder.push_bind(Uuid::from(journal_id));
                builder.push_bind(entry_type);
                builder.push_bind(layer);
                builder.push_bind(units);
                builder.push_bind(currency);
                builder.push_bind(direction);
                builder.push_bind(description);
                builder.push_bind(sequence);
                builder.push("(SELECT id FROM accounts WHERE id = ");
                builder.push_bind_unseparated(Uuid::from(account_id));
                builder.push_unseparated(")");
                sequence += 1;
            },
        );
        let query = query_builder.build();
        query.execute(&mut tx).await?;
        Ok(tx)
    }
}
