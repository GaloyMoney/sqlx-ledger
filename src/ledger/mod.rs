use sqlx::{Pool, Postgres};

use crate::account::Accounts;

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
}
