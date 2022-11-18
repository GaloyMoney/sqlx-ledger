use sqlx::{Pool, Postgres};

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
            name,
            description,
            status,
        }: NewJournal,
    ) -> Result<JournalId, SqlxLedgerError> {
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_journals (id, version, name, description, status)
            VALUES (gen_random_uuid(), 1, $1, $2, $3)
            RETURNING id, version, created_at"#,
            name,
            description,
            status as Status,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(JournalId::from(record.id))
    }
}
