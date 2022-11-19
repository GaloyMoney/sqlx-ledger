use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::entity::*;
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
            id,
            code,
            name,
            normal_balance_type,
            description,
            status,
            metadata,
        }: NewAccount,
    ) -> Result<AccountId, SqlxLedgerError> {
        let record = sqlx::query!(
            r#"INSERT INTO sqlx_ledger_accounts (id, code, name, normal_balance_type, description, status, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, version, created_at"#,
            Uuid::from(id),
            code,
            name,
            normal_balance_type as DebitOrCredit,
            description,
            status as Status,
            metadata
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(AccountId::from(record.id))
    }

    pub async fn update<T: Serialize>(
        &self,
        id: AccountId,
        description: Option<String>,
        metadata: Option<T>,
    ) -> Result<AccountId, SqlxLedgerError> {
        let metadata_json = match metadata {
            Some(m) => Some(serde_json::to_value(m)?),
            None => None,
        };
        sqlx::query_file!(
            "src/account/sql/update-account.sql",
            Uuid::from(id),
            description,
            metadata_json
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }
}
