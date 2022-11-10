use sqlx::{Pool, Postgres};

use super::new_account::*;
use crate::{error::*, primitives::*};

pub struct Accounts {
    pool: Pool<Postgres>,
}

impl Accounts {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(
        &self,
        NewAccount {
            code,
            name,
            normal_balance_type,
            description,
            status,
            metadata,
        }: NewAccount,
    ) -> Result<AccountId, SqlxLedgerError> {
        let mut tx = self.pool.begin().await?;
        let record = sqlx::query!(
            r#"INSERT INTO accounts_current (id, version, code, name, normal_balance_type, description, status, metadata)
            VALUES (gen_random_uuid(), 1, $1, $2, $3, $4, $5, $6)
            RETURNING id, version, created_at"#,
            code,
            name,
            normal_balance_type as DebitOrCredit,
            description,
            status as Status,
            metadata
        )
        .fetch_one(&mut tx)
        .await?;
        sqlx::query!(
            r#"INSERT INTO accounts_history (id, version, code, name, normal_balance_type, description, status, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            record.id,
            record.version,
            code,
            name,
            normal_balance_type as DebitOrCredit,
            description,
            status as Status,
            metadata
        )
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(AccountId::from(record.id))
    }
}
