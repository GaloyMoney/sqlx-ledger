use sqlx::PgPool;

use crate::{account::Accounts, entry::*, error::*, journal::*, transaction::*, tx_template::*};

pub struct SqlxLedger {
    pool: PgPool,
    accounts: Accounts,
    journals: Journals,
    tx_templates: TxTemplates,
    transactions: Transactions,
    entries: Entries,
}

impl SqlxLedger {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            accounts: Accounts::new(pool),
            journals: Journals::new(pool),
            tx_templates: TxTemplates::new(pool),
            transactions: Transactions::new(pool),
            entries: Entries::new(pool),
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

    pub fn entries(&self) -> &Entries {
        &self.entries
    }

    pub async fn post_transaction(
        &self,
        tx_template_code: String,
        params: Option<TxParams>,
    ) -> Result<(), SqlxLedgerError> {
        let tx_template = self.tx_templates.find_core(tx_template_code).await?;
        let (new_tx, new_entries) = tx_template.prep_tx(params.unwrap_or_else(TxParams::new))?;
        let (journal_id, tx_id, tx) = self.transactions.create(new_tx).await?;
        let (entries, tx) = self
            .entries
            .create_all(journal_id, tx_id, new_entries, tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
