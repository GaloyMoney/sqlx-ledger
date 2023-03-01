use sqlx::{Pool, Postgres, Transaction};
use tracing::instrument;
use uuid::Uuid;

use super::entity::*;
use crate::{error::*, primitives::*};

/// Repository for working with `Journal` entities.
#[derive(Debug, Clone)]
pub struct Journals {
    pool: Pool<Postgres>,
}

impl Journals {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, new_journal: NewJournal) -> Result<JournalId, SqlxLedgerError> {
        let mut tx = self.pool.begin().await?;
        let res = self.create_in_tx(&mut tx, new_journal).await?;
        tx.commit().await?;
        Ok(res)
    }

    #[instrument(name = "sqlx_ledger.journals.create", skip(self, tx))]
    pub async fn create_in_tx<'a>(
        &self,
        tx: &mut Transaction<'a, Postgres>,
        new_journal: NewJournal,
    ) -> Result<JournalId, SqlxLedgerError> {
        let NewJournal {
            id,
            name,
            description,
            status,
        } = new_journal;
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_journals (id, name, description, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, version, created_at"#,
            Uuid::from(id),
            name,
            description,
            status as Status,
        )
        .fetch_one(&mut *tx)
        .await?;
        Ok(JournalId::from(record.id))
    }
}
