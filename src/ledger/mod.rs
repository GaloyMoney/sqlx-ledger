use sqlx::{Pool, Postgres};

use crate::{account::Accounts, transaction::*, tx_template::TxTemplates};

pub struct SqlxLedger {
    pool: Pool<Postgres>,
}

impl SqlxLedger {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub fn accounts(&self) -> Accounts {
        Accounts::new(&self.pool)
    }

    pub fn tx_templates(&self) -> TxTemplates {
        TxTemplates::new(&self.pool)
    }

    pub async fn post_transaction(
        &self,
        tx_template_code: String,
        params: Option<TxParams>,
    ) -> Result<(), ()> {
        Ok(())
    }
}
