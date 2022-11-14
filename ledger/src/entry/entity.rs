use derive_builder::Builder;
use rust_decimal::Decimal;

use crate::primitives::*;

#[derive(Builder)]
pub(crate) struct NewEntry {
    pub(super) account_id: AccountId,
    pub(super) entry_type: String,
    pub(super) layer: Layer,
    pub(super) units: Decimal,
    pub(super) currency: String,
    pub(super) direction: DebitOrCredit,
    #[builder(setter(strip_option), default)]
    pub(super) description: Option<String>,
}

impl NewEntry {
    pub fn builder() -> NewEntryBuilder {
        NewEntryBuilder::default()
    }
}
