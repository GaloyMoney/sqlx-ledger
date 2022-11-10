use sqlx::{Pool, Postgres};

use super::{account::*, new_account::*};
use crate::{error::*, primitives::*};

pub struct Accounts {
    pool: Pool<Postgres>,
}

impl Accounts {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self { pool: pool.clone() }
    }

    pub fn create<M>(new_account: NewAccount) -> Result<Account<M>, SqlxLedgerError> {
        unimplemented!()
    }
}
