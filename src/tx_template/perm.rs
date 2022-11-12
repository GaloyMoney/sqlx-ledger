use crate::primitives::*;

use super::{entity::TxInput, param_definition::ParamDefinition};

pub(crate) struct TxTemplatePerm {
    pub(super) id: TxTemplateId,
    pub(super) code: String,
    pub(super) params: Option<Vec<ParamDefinition>>,
    pub(super) tx_input: TxInput,
}
