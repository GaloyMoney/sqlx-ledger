use crate::primitives::*;
use chrono::{DateTime, NaiveDate, Utc};
use derive_builder::Builder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: TransactionId,
    pub version: u32,
    pub journal_id: JournalId,
    pub tx_template_id: TxTemplateId,
    pub effective: NaiveDate,
    pub correlation_id: CorrelationId,
    pub external_id: String,
    pub description: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata_json: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl Transaction {
    pub fn metadata<T: DeserializeOwned>(&self) -> Result<Option<T>, serde_json::Error> {
        match self.metadata_json.as_ref() {
            Some(json) => Ok(serde_json::from_value(json.clone())?),
            None => Ok(None),
        }
    }
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
