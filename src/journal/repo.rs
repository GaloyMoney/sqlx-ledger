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
        let mut tx = self.pool.begin().await?;
        let record = sqlx::query!(
            r#"INSERT INTO journals_current (id, version, name, description, status)
            VALUES (gen_random_uuid(), 1, $1, $2, $3)
            RETURNING id, version, created_at"#,
            name,
            description,
            status as Status,
        )
        .fetch_one(&mut tx)
        .await?;
        sqlx::query!(
            r#"INSERT INTO journals_history (id, version, name, description, status)
            VALUES ($1, $2, $3, $4, $5)"#,
            record.id,
            record.version,
            name,
            description,
            status as Status,
        )
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(JournalId::from(record.id))
    }
}
