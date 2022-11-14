use crate::primitives::*;
use chrono::NaiveDate;
use derive_builder::Builder;

#[derive(Builder)]
pub(crate) struct NewTransaction {
    pub(super) journal_id: JournalId,
    pub(super) tx_template_id: TxTemplateId,
    pub(super) effective: NaiveDate,
    #[builder(setter(strip_option), default)]
    pub(super) correlation_id: Option<CorrelationId>,
    #[builder(setter(strip_option), default)]
    pub(super) external_id: Option<ExternalId>,
    #[builder(setter(strip_option), default)]
    pub(super) description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub(super) metadata: Option<serde_json::Value>,
}

impl NewTransaction {
    pub fn builder() -> NewTransactionBuilder {
        NewTransactionBuilder::default()
    }
}
