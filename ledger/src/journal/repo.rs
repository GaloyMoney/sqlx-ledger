use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::entity::*;
use crate::{error::*, primitives::*};

pub struct Journals {
    pool: Pool<Postgres>,
}

impl Journals {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(
        &self,
        NewJournal {
            id,
            name,
            description,
            status,
        }: NewJournal,
    ) -> Result<JournalId, SqlxLedgerError> {
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_journals (id, name, description, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, version, created_at"#,
            Uuid::from(id),
            name,
            description,
            status as Status,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(JournalId::from(record.id))
    }
}
