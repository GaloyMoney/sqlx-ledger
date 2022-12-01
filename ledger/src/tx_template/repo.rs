use sqlx::{Pool, Postgres};

use super::{core::*, entity::*};
use crate::{error::*, primitives::*};

#[derive(Debug, Clone)]
pub struct TxTemplates {
    pool: Pool<Postgres>,
}

impl TxTemplates {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(
        &self,
        NewTxTemplate {
            code,
            description,
            params,
            tx_input,
            entries,
            metadata,
        }: NewTxTemplate,
    ) -> Result<TxTemplateId, SqlxLedgerError> {
        let params_json = serde_json::to_value(&params)?;
        let tx_input_json = serde_json::to_value(&tx_input)?;
        let entries_json = serde_json::to_value(&entries)?;
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_tx_templates (id, code, description, params, tx_input, entries, metadata)
            VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6)
            RETURNING id, version, created_at"#,
            code,
            description,
            params_json,
            tx_input_json,
            entries_json,
            metadata
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(TxTemplateId::from(record.id))
    }

    pub(crate) async fn find_core(&self, code: &str) -> Result<TxTemplateCore, SqlxLedgerError> {
        let record = sqlx::query!(
            r#"SELECT id, code, params, tx_input, entries FROM sqlx_ledger_tx_templates WHERE code = $1 LIMIT 1"#,
            code
        )
        .fetch_one(&self.pool)
        .await?;
        let params = match record.params {
            Some(params) => Some(serde_json::from_value(params)?),
            None => None,
        };
        let tx_input = serde_json::from_value(record.tx_input)?;
        Ok(TxTemplateCore {
            id: TxTemplateId::from(record.id),
            _code: record.code,
            params,
            entries: serde_json::from_value(record.entries)?,
            tx_input,
        })
    }
}
