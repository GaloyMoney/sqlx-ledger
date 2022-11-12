use sqlx::{Pool, Postgres};

use crate::{account::Accounts, error::*, transaction::*, tx_template::*};

pub struct SqlxLedger {
    pool: Pool<Postgres>,
    accounts: Accounts,
    tx_templates: TxTemplates,
}

impl SqlxLedger {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self {
            accounts: Accounts::new(pool),
            tx_templates: TxTemplates::new(pool),
            pool: pool.clone(),
        }
    }

    pub fn accounts(&self) -> &Accounts {
        &self.accounts
    }

    pub fn tx_templates(&self) -> &TxTemplates {
        &self.tx_templates
    }

    pub async fn post_transaction(
        &self,
        tx_template_code: String,
        params: Option<TxParams>,
    ) -> Result<(), SqlxLedgerError> {
        let tx_template = self.tx_templates.find_perm(tx_template_code).await?;
        // tx_template.prep_tx(params);
        Ok(())
    }
}
