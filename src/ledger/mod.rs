use sqlx::{Pool, Postgres};

use crate::{account::Accounts, tx_template::TxTemplates};

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
}
