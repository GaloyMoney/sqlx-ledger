use crate::primitives::*;
use chrono::NaiveDate;

pub(crate) struct NewTransaction {
    journal_id: JournalId,
    tx_template_id: TxTemplateId,
    correlation_id: Option<CorrelationId>,
    external_id: Option<ExternalId>,
    effective: NaiveDate,
    description: Option<String>,
    metadata: Option<serde_json::Value>,
}
