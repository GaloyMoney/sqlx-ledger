use serde::Deserialize;

use crate::{cel::CelExpression, primitives::*, transaction::NewTransaction};

use super::{entity::TxInput, param_definition::ParamDefinition, tx_params::TxParams};

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
    pub(crate) fn prep_tx(&self, params: Option<TxParams>) -> Result<NewTransaction, String> {
        let mut tx_builder = NewTransaction::builder();
        // let journal_id = self.tx_input.journal_id.evaluate(params);
        unimplemented!()
    }
}
