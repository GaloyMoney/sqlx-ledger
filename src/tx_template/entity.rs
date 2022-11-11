use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::cel::CelExpression;

use super::param::*;

#[derive(Builder)]
pub struct NewTxTemplate {
    #[builder(setter(into))]
    pub(super) code: String,
    #[builder(setter(strip_option, into), default)]
    pub(super) description: Option<String>,
    #[builder(default)]
    pub(super) params: Vec<ParamDefinition>,
    pub(super) tx_input: TxInput,
    #[builder(setter(custom), default)]
    pub(super) metadata: Option<serde_json::Value>,
}

impl NewTxTemplate {
    pub fn builder() -> NewTxTemplateBuilder {
        NewTxTemplateBuilder::default()
    }
}

impl NewTxTemplateBuilder {
    pub fn metadata<T: serde::Serialize>(
        &mut self,
        metadata: T,
    ) -> Result<&mut Self, serde_json::Error> {
        self.metadata = Some(Some(serde_json::to_value(metadata)?));
        Ok(self)
    }
}

#[derive(Clone, Deserialize, Serialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct TxInput {
    #[builder(setter(into))]
    effective: String,
    #[builder(setter(into))]
    journal_id: String,
    #[builder(default)]
    correlation_id: Option<String>,
    #[builder(default)]
    external_id: Option<String>,
    #[builder(default)]
    description: Option<String>,
    #[builder(default)]
    metadata: Option<String>,
}

impl TxInput {
    pub fn builder() -> TxInputBuilder {
        TxInputBuilder::default()
    }
}

impl TxInputBuilder {
    fn validate(&self) -> Result<(), String> {
        let _ = CelExpression::try_from(self.effective.as_ref().unwrap().as_str())
            .map_err(|e| e.to_string())?;
        let _ = CelExpression::try_from(self.journal_id.as_ref().unwrap().as_str())
            .map_err(|e| e.to_string())?;
        if let Some(Some(expr)) = self.correlation_id.as_ref() {
            let _ = CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
        }
        if let Some(Some(expr)) = self.external_id.as_ref() {
            let _ = CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
        }
        if let Some(Some(expr)) = self.description.as_ref() {
            let _ = CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
        }
        if let Some(Some(expr)) = self.metadata.as_ref() {
            let _ = CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn it_builds() {
        let journal_id = Uuid::new_v4();
        let new_journal = NewTxTemplate::builder()
            .code("CODE")
            .tx_input(
                TxInput::builder()
                    .effective("'2022-11-01'")
                    .journal_id(format!("'{}'", journal_id))
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        assert_eq!(new_journal.description, None);
    }

    #[test]
    fn fails_when_mandatory_fields_are_missing() {
        let new_account = NewTxTemplate::builder().build();
        assert!(new_account.is_err());
    }
}
