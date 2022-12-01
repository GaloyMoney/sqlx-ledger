use sqlx::{Pool, Postgres, Transaction};
use uuid::Uuid;

use super::entity::*;
use crate::{error::*, primitives::*};

#[derive(Debug, Clone)]
pub struct Transactions {
    pool: Pool<Postgres>,
}

impl Transactions {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub(crate) async fn create<'a>(
        &self,
        NewTransaction {
            journal_id,
            tx_template_id,
            effective,
            correlation_id,
            external_id,
            description,
            metadata,
        }: NewTransaction,
    ) -> Result<(JournalId, TransactionId, Transaction<'a, Postgres>), SqlxLedgerError> {
        let mut tx = self.pool.begin().await?;
        let id = Uuid::new_v4();
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_transactions (id, version, journal_id, tx_template_id, effective, correlation_id, external_id, description, metadata)
            VALUES ($1, 1, (SELECT id FROM sqlx_ledger_journals WHERE id = $2 LIMIT 1), (SELECT id FROM sqlx_ledger_tx_templates WHERE id = $3 LIMIT 1), $4, $5, $6, $7, $8)
            RETURNING id, version, created_at"#,
            id,
            Uuid::from(journal_id),
            Uuid::from(tx_template_id),
            effective,
            correlation_id.map(Uuid::from).unwrap_or(id),
            external_id.map(Uuid::from).unwrap_or(id),
            description,
            metadata
        )
        .fetch_one(&mut tx)
        .await?;
        Ok((journal_id, TransactionId::from(record.id), tx))
    }
}
