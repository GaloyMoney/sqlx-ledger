use crate::primitives::*;

use super::{entity::TxInput, param_definition::ParamDefinition, tx_params::TxParams};
use crate::transaction::NewTransaction;

pub(crate) struct TxTemplateCore {
    pub(super) id: TxTemplateId,
    pub(super) code: String,
    pub(super) params: Option<Vec<ParamDefinition>>,
    pub(super) tx_input: TxInput,
}

impl TxTemplateCore {
    pub(crate) fn prep_tx(&self, params: Option<TxParams>) -> Result<NewTransaction, String> {
        let mut tx_builder = NewTransaction::builder();
        // let mut tx_input = self.tx_input.clone();
        unimplemented!()
    }
}
