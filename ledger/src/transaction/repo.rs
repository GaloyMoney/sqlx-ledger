use sqlx::{Pool, Postgres, Transaction as DbTransaction};
use tracing::instrument;
use uuid::Uuid;

use super::entity::*;
use crate::{error::*, primitives::*};

/// Repository for working with `TxTemplate` entities.
#[derive(Debug, Clone)]
pub struct Transactions {
    pool: Pool<Postgres>,
}

impl Transactions {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    #[instrument(level = "trace", name = "sqlx_ledger.transactions.create_in_tx")]
    pub(crate) async fn create_in_tx(
        &self,
        tx: &mut DbTransaction<'_, Postgres>,
        tx_id: TransactionId,
        NewTransaction {
            journal_id,
            tx_template_id,
            effective,
            correlation_id,
            external_id,
            description,
            metadata,
        }: NewTransaction,
    ) -> Result<(JournalId, TransactionId), SqlxLedgerError> {
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_transactions (id, version, journal_id, tx_template_id, effective, correlation_id, external_id, description, metadata)
            VALUES ($1, 1, (SELECT id FROM sqlx_ledger_journals WHERE id = $2 LIMIT 1), (SELECT id FROM sqlx_ledger_tx_templates WHERE id = $3 LIMIT 1), $4, $5, $6, $7, $8)
            RETURNING id, version, created_at"#,
            tx_id as TransactionId,
            journal_id as JournalId,
            tx_template_id as TxTemplateId,
            effective,
            correlation_id.map(Uuid::from).unwrap_or(Uuid::from(tx_id)),
            external_id.unwrap_or_else(|| tx_id.to_string()),
            description,
            metadata
        )
        .fetch_one(&mut *tx)
        .await?;
        Ok((journal_id, TransactionId::from(record.id)))
    }

    pub async fn list_by_external_ids(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<Transaction>, SqlxLedgerError> {
        let records = sqlx::query!(
            r#"SELECT id, version, journal_id, tx_template_id, effective, correlation_id, external_id, description, metadata, created_at, modified_at
            FROM sqlx_ledger_transactions
            WHERE external_id = ANY($1)"#,
            &ids[..]
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records
            .into_iter()
            .map(|row| Transaction {
                id: TransactionId::from(row.id),
                version: row.version as u32,
                journal_id: JournalId::from(row.journal_id),
                tx_template_id: TxTemplateId::from(row.tx_template_id),
                effective: row.effective,
                correlation_id: CorrelationId::from(row.correlation_id),
                external_id: row.external_id,
                description: row.description,
                metadata_json: row.metadata,
                created_at: row.created_at,
                modified_at: row.modified_at,
            })
            .collect())
    }

    pub async fn list_by_ids(
        &self,
        ids: impl IntoIterator<Item = impl std::borrow::Borrow<TransactionId>>,
    ) -> Result<Vec<Transaction>, SqlxLedgerError> {
        let ids: Vec<_> = ids.into_iter().map(|id| Uuid::from(id.borrow())).collect();
        let records = sqlx::query!(
            r#"SELECT id, version, journal_id, tx_template_id, effective, correlation_id, external_id, description, metadata, created_at, modified_at
            FROM sqlx_ledger_transactions
            WHERE id = ANY($1)"#,
            &ids[..]
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records
            .into_iter()
            .map(|row| Transaction {
                id: TransactionId::from(row.id),
                version: row.version as u32,
                journal_id: JournalId::from(row.journal_id),
                tx_template_id: TxTemplateId::from(row.tx_template_id),
                effective: row.effective,
                correlation_id: CorrelationId::from(row.correlation_id),
                external_id: row.external_id,
                description: row.description,
                metadata_json: row.metadata,
                created_at: row.created_at,
                modified_at: row.modified_at,
            })
            .collect())
    }
}
