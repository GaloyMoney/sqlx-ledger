use chrono::{DateTime, Utc};
use derive_builder::Builder;
use rust_decimal::Decimal;

use crate::primitives::*;

pub struct Entry {
    pub id: EntryId,
    pub version: u32,
    pub transaction_id: TransactionId,
    pub account_id: AccountId,
    pub journal_id: JournalId,
    pub entry_type: String,
    pub layer: Layer,
    pub units: Decimal,
    pub currency: Currency,
    pub direction: DebitOrCredit,
    pub sequence: u32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Builder)]
pub(crate) struct NewEntry {
    pub(super) account_id: AccountId,
    pub(super) entry_type: String,
    pub(super) layer: Layer,
    pub(super) units: Decimal,
    pub(super) currency: Currency,
    pub(super) direction: DebitOrCredit,
    #[builder(setter(strip_option), default)]
    pub(super) description: Option<String>,
}

impl NewEntry {
    pub fn builder() -> NewEntryBuilder {
        NewEntryBuilder::default()
    }
}
