use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::entry::StagedEntry;
use crate::primitives::*;

#[derive(Debug, Clone)]
pub struct Balance {
    pub journal_id: JournalId,
    pub account_id: AccountId,
    pub entry_id: EntryId,
    pub currency: Currency,
    pub settled_dr_balance: Decimal,
    pub settled_cr_balance: Decimal,
    pub settled_entry_id: EntryId,
    pub settled_modified_at: DateTime<Utc>,
    pub pending_dr_balance: Decimal,
    pub pending_cr_balance: Decimal,
    pub pending_entry_id: EntryId,
    pub pending_modified_at: DateTime<Utc>,
    pub encumbered_dr_balance: Decimal,
    pub encumbered_cr_balance: Decimal,
    pub encumbered_entry_id: EntryId,
    pub encumbered_modified_at: DateTime<Utc>,
    pub version: i32,
    pub modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Balance {
    pub(crate) fn update(&self, entry: &StagedEntry) -> Self {
        self.clone().update_inner(entry)
    }

    pub(crate) fn init(journal_id: JournalId, entry: &StagedEntry) -> Self {
        Self {
            journal_id,
            account_id: entry.account_id,
            entry_id: entry.entry_id,
            currency: entry.currency,
            settled_dr_balance: Decimal::ZERO,
            settled_cr_balance: Decimal::ZERO,
            settled_entry_id: entry.entry_id,
            settled_modified_at: entry.created_at,
            pending_dr_balance: Decimal::ZERO,
            pending_cr_balance: Decimal::ZERO,
            pending_entry_id: entry.entry_id,
            pending_modified_at: entry.created_at,
            encumbered_dr_balance: Decimal::ZERO,
            encumbered_cr_balance: Decimal::ZERO,
            encumbered_entry_id: entry.entry_id,
            encumbered_modified_at: entry.created_at,
            version: 0,
            modified_at: entry.created_at,
            created_at: entry.created_at,
        }
        .update_inner(entry)
    }

    fn update_inner(mut self, entry: &StagedEntry) -> Self {
        self.version += 1;
        self.modified_at = entry.created_at;
        self.entry_id = entry.entry_id;
        match entry.layer {
            Layer::Settled => {
                self.settled_entry_id = entry.entry_id;
                self.settled_modified_at = entry.created_at;
                match entry.direction {
                    DebitOrCredit::Debit => {
                        self.settled_dr_balance += entry.units;
                    }
                    DebitOrCredit::Credit => {
                        self.settled_cr_balance += entry.units;
                    }
                }
            }
            Layer::Pending => {
                self.pending_entry_id = entry.entry_id;
                self.pending_modified_at = entry.created_at;
                match entry.direction {
                    DebitOrCredit::Debit => {
                        self.pending_dr_balance += entry.units;
                    }
                    DebitOrCredit::Credit => {
                        self.pending_cr_balance += entry.units;
                    }
                }
            }
            Layer::Encumbered => {
                self.encumbered_entry_id = entry.entry_id;
                self.encumbered_modified_at = entry.created_at;
                match entry.direction {
                    DebitOrCredit::Debit => {
                        self.encumbered_dr_balance += entry.units;
                    }
                    DebitOrCredit::Credit => {
                        self.encumbered_cr_balance += entry.units;
                    }
                }
            }
        }
        self
    }
}
