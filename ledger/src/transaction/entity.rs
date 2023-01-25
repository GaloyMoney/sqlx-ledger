use crate::primitives::*;
use chrono::{DateTime, NaiveDate, Utc};
use derive_builder::Builder;

pub struct Transaction {
    pub id: TransactionId,
    pub version: u32,
    pub journal_id: JournalId,
    pub tx_template_id: TxTemplateId,
    pub effective: NaiveDate,
    pub correlation_id: CorrelationId,
    pub external_id: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Builder)]
pub(crate) struct NewTransaction {
    #[builder(setter(into))]
    pub(super) journal_id: JournalId,
    pub(super) tx_template_id: TxTemplateId,
    pub(super) effective: NaiveDate,
    #[builder(setter(strip_option), default)]
    pub(super) correlation_id: Option<CorrelationId>,
    #[builder(setter(strip_option), default)]
    pub(super) external_id: Option<String>,
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
