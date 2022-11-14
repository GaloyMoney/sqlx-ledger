use sqlx::{Pool, Postgres};

use crate::{account::Accounts, error::*, journal::*, transaction::*, tx_template::*};

pub struct SqlxLedger {
    pool: Pool<Postgres>,
    accounts: Accounts,
    journals: Journals,
    tx_templates: TxTemplates,
    transactions: Transactions,
}

impl SqlxLedger {
    pub fn new(pool: &Pool<Postgres>) -> Self {
        Self {
            accounts: Accounts::new(pool),
            journals: Journals::new(pool),
            tx_templates: TxTemplates::new(pool),
            transactions: Transactions::new(pool),
            pool: pool.clone(),
        }
    }

    pub fn accounts(&self) -> &Accounts {
        &self.accounts
    }

    pub fn journals(&self) -> &Journals {
        &self.journals
    }

    pub fn tx_templates(&self) -> &TxTemplates {
        &self.tx_templates
    }

    pub async fn post_transaction(
        &self,
        tx_template_code: String,
        params: Option<TxParams>,
    ) -> Result<(), SqlxLedgerError> {
        let tx_template = self.tx_templates.find_core(tx_template_code).await?;
        let new_tx = tx_template.prep_tx(params)?;
        let (_, tx) = self.transactions.create(new_tx).await?;
        tx.commit().await?;
        Ok(())
    }
}
