use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::*, primitives::*, transaction::NewTransaction};
use cel_interpreter::{CelContext, CelExpression};

use super::{param_definition::ParamDefinition, tx_params::TxParams};

#[derive(Deserialize)]
pub(crate) struct TxInputCel {
    effective: CelExpression,
    journal_id: CelExpression,
    correlation_id: Option<CelExpression>,
    external_id: Option<CelExpression>,
    description: Option<CelExpression>,
    metadata: Option<CelExpression>,
}

pub(crate) struct TxTemplateCore {
    pub(super) id: TxTemplateId,
    pub(super) code: String,
    pub(super) params: Option<Vec<ParamDefinition>>,
    pub(super) tx_input: TxInputCel,
}

impl TxTemplateCore {
    pub(crate) fn prep_tx(
        &self,
        params: Option<TxParams>,
    ) -> Result<NewTransaction, SqlxLedgerError> {
        let mut tx_builder = NewTransaction::builder();
        tx_builder.tx_template_id(self.id);

        let ctx = params
            .map(CelContext::from)
            .unwrap_or_else(|| CelContext::new());

        let journal_id: Uuid = self.tx_input.journal_id.try_evaluate(&ctx)?;
        tx_builder.journal_id(journal_id.into());

        let effective: NaiveDate = self.tx_input.effective.try_evaluate(&ctx)?;
        tx_builder.effective(effective);

        if let Some(correlation_id) = self.tx_input.correlation_id.as_ref() {
            let correlation_id: Uuid = correlation_id.try_evaluate(&ctx)?;
            tx_builder.correlation_id(correlation_id.into());
        }

        if let Some(external_id) = self.tx_input.external_id.as_ref() {
            let external_id: Uuid = external_id.try_evaluate(&ctx)?;
            tx_builder.external_id(external_id.into());
        }

        if let Some(description) = self.tx_input.description.as_ref() {
            let description: String = description.try_evaluate(&ctx)?;
            tx_builder.description(description);
        }

        if let Some(metadata) = self.tx_input.metadata.as_ref() {
            let metadata: serde_json::Value = metadata.try_evaluate(&ctx)?;
            tx_builder.metadata(metadata);
        }

        Ok(tx_builder.build().expect("tx_build should succeed"))
    }
}
