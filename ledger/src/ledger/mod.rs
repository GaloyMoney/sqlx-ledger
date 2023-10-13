use sqlx::{Acquire, PgPool, Postgres, Transaction};
use tracing::instrument;

use std::collections::HashMap;

use crate::{
    account::Accounts, balance::*, entry::*, error::*, event::*, journal::*, primitives::*,
    transaction::*, tx_template::*,
};

#[derive(Debug, Clone)]
pub struct SqlxLedger {
    pool: PgPool,
    accounts: Accounts,
    journals: Journals,
    tx_templates: TxTemplates,
    transactions: Transactions,
    entries: Entries,
    balances: Balances,
}

impl SqlxLedger {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            accounts: Accounts::new(pool),
            journals: Journals::new(pool),
            tx_templates: TxTemplates::new(pool),
            transactions: Transactions::new(pool),
            entries: Entries::new(pool),
            balances: Balances::new(pool),
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

    pub fn balances(&self) -> &Balances {
        &self.balances
    }

    pub fn transactions(&self) -> &Transactions {
        &self.transactions
    }

    pub async fn post_transaction(
        &self,
        tx_id: TransactionId,
        tx_template_code: &str,
        params: Option<impl Into<TxParams> + std::fmt::Debug>,
    ) -> Result<(), SqlxLedgerError> {
        let tx = self.pool.begin().await?;
        self.post_transaction_in_tx(tx, tx_id, tx_template_code, params)
            .await?;
        Ok(())
    }

    #[instrument(name = "sqlx_ledger.ledger.post_transaction", skip(self, tx))]
    pub async fn post_transaction_in_tx(
        &self,
        mut tx: Transaction<'_, Postgres>,
        tx_id: TransactionId,
        tx_template_code: &str,
        params: Option<impl Into<TxParams> + std::fmt::Debug>,
    ) -> Result<(), SqlxLedgerError> {
        let tx_template = self.tx_templates.find_core(tx_template_code).await?;
        let (new_tx, new_entries) =
            tx_template.prep_tx(params.map(|p| p.into()).unwrap_or_default())?;
        let (journal_id, tx_id) = self
            .transactions
            .create_in_tx(&mut tx, tx_id, new_tx)
            .await?;
        let entries = self
            .entries
            .create_all(journal_id, tx_id, new_entries, &mut tx)
            .await?;
        {
            let ids: Vec<(AccountId, &Currency)> = entries
                .iter()
                .map(|entry| (entry.account_id, &entry.currency))
                .collect();
            let mut balance_tx = tx.begin().await?;

            let mut balances = self
                .balances
                .find_for_update(journal_id, ids.clone(), &mut balance_tx)
                .await?;
            let mut latest_balances: HashMap<(AccountId, &Currency), BalanceDetails> =
                HashMap::new();
            let mut new_balances = Vec::new();
            for entry in entries.iter() {
                let balance = match (
                    latest_balances.remove(&(entry.account_id, &entry.currency)),
                    balances.remove(&(entry.account_id, entry.currency)),
                ) {
                    (Some(latest), _) => {
                        new_balances.push(latest.clone());
                        latest
                    }
                    (_, Some(balance)) => balance,
                    _ => {
                        latest_balances.insert(
                            (entry.account_id, &entry.currency),
                            BalanceDetails::init(journal_id, entry),
                        );
                        continue;
                    }
                };
                latest_balances.insert((entry.account_id, &entry.currency), balance.update(entry));
            }
            new_balances.extend(latest_balances.into_values());

            self.balances
                .update_balances(journal_id, new_balances, &mut balance_tx)
                .await?;
            balance_tx.commit().await?;
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn events(
        &self,
        opts: EventSubscriberOpts,
    ) -> Result<EventSubscriber, SqlxLedgerError> {
        EventSubscriber::connect(&self.pool, opts).await
    }
}
