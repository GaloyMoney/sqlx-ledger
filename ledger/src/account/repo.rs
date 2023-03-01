use serde::Serialize;
use sqlx::{Pool, Postgres, Transaction};
use tracing::instrument;
use uuid::Uuid;

use super::entity::*;
use crate::{error::*, primitives::*};

/// Repository for working with `Account` entities.
#[derive(Debug, Clone)]
pub struct Accounts {
    pool: Pool<Postgres>,
}

impl Accounts {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create(&self, new_account: NewAccount) -> Result<AccountId, SqlxLedgerError> {
        let mut tx = self.pool.begin().await?;
        let res = self.create_in_tx(&mut tx, new_account).await?;
        tx.commit().await?;
        Ok(res)
    }

    #[instrument(name = "sqlx_ledger.accounts.create", skip(self, tx))]
    pub async fn create_in_tx<'a>(
        &self,
        tx: &mut Transaction<'a, Postgres>,
        new_account: NewAccount,
    ) -> Result<AccountId, SqlxLedgerError> {
        let NewAccount {
            id,
            code,
            name,
            normal_balance_type,
            description,
            status,
            metadata,
        } = new_account;
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
        .fetch_one(&mut *tx)
        .await?;
        Ok(AccountId::from(record.id))
    }

    #[instrument(name = "sqlx_ledger.accounts.update", skip(self))]
    pub async fn update<T: Serialize + std::fmt::Debug>(
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

    #[instrument(name = "sqlx_ledger.accounts.find_by_code", skip(self))]
    pub async fn find_by_code(&self, code: &str) -> Result<Option<AccountId>, SqlxLedgerError> {
        let record = sqlx::query!(
            r#"SELECT id FROM sqlx_ledger_accounts WHERE code = $1 LIMIT 1"#,
            code
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(record.map(|r| AccountId::from(r.id)))
    }
}
