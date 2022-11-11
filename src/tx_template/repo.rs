use sqlx::{Pool, Postgres};

use super::entity::*;
use crate::{error::*, primitives::*};

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
            metadata,
        }: NewTxTemplate,
    ) -> Result<TxTemplateId, SqlxLedgerError> {
        let params_json = serde_json::to_value(&params)?;
        let tx_input_json = serde_json::to_value(&tx_input)?;
        let mut tx = self.pool.begin().await?;
        let record = sqlx::query!(
            r#"INSERT INTO tx_templates_current (id, version, code, description, params, tx_input, metadata)
            VALUES (gen_random_uuid(), 1, $1, $2, $3, $4, $5)
            RETURNING id, version, created_at"#,
            code,
            description,
            params_json,
            tx_input_json,
            metadata
        )
        .fetch_one(&mut tx)
        .await?;
        sqlx::query!(
            r#"INSERT INTO tx_templates_history (id, version, code, description, params, tx_input, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            record.id,
            record.version,
            code,
            description,
            params_json,
            tx_input_json,
            metadata
        )
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(TxTemplateId::from(record.id))
    }
}
