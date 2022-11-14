use derive_builder::Builder;
use serde::Serialize;

use cel_interpreter::CelExpression;

use super::param_definition::*;

#[derive(Builder)]
pub struct NewTxTemplate {
    #[builder(setter(into))]
    pub(super) code: String,
    #[builder(setter(strip_option, into), default)]
    pub(super) description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub(super) params: Option<Vec<ParamDefinition>>,
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

#[derive(Clone, Serialize, Builder)]
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
        let _ = validate_expression(&self.effective)?;
        let _ = validate_expression(&self.journal_id)?;
        let _ = validate_optional_expression(&self.correlation_id)?;
        let _ = validate_optional_expression(&self.external_id)?;
        let _ = validate_optional_expression(&self.description)?;
        validate_optional_expression(&self.metadata)
    }
}

#[derive(Clone, Serialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct EntryInput {
    #[builder(setter(into))]
    entry_type: String,
    #[builder(setter(into))]
    account_id: String,
    #[builder(setter(into))]
    layer: String,
    #[builder(setter(into))]
    direction: String,
    #[builder(setter(into))]
    units: String,
    #[builder(setter(into))]
    currency: String,
    #[builder(setter(strip_option), default)]
    description: Option<String>,
}

impl EntryInputBuilder {
    fn validate(&self) -> Result<(), String> {
        let _ = validate_expression(&self.entry_type)?;
        let _ = validate_expression(&self.account_id)?;
        let _ = validate_expression(&self.layer)?;
        let _ = validate_expression(&self.direction)?;
        let _ = validate_expression(&self.units)?;
        let _ = validate_expression(&self.currency)?;
        validate_optional_expression(&self.description)
    }
}

fn validate_expression(expr: &Option<String>) -> Result<(), String> {
    let _ = CelExpression::try_from(expr.as_ref().unwrap().as_str()).map_err(|e| e.to_string())?;
    Ok(())
}
fn validate_optional_expression(expr: &Option<Option<String>>) -> Result<(), String> {
    if let Some(Some(expr)) = expr.as_ref() {
        let _ = CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
    }
    Ok(())
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
