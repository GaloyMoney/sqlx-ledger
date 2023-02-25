use derive_builder::Builder;
use serde::Serialize;

use cel_interpreter::CelExpression;

use super::param_definition::*;
use crate::primitives::*;

/// Representation of a new TxTemplateCore created via a builder.
///
/// TxTemplateCore is an entity that takes a set of params including
/// a `TxInput` entity and a set of `EntryInput` entities. It can
/// later be used to create a `Transaction`.
#[derive(Builder)]
pub struct NewTxTemplate {
    #[builder(setter(into))]
    pub(super) id: TxTemplateId,
    #[builder(setter(into))]
    pub(super) code: String,
    #[builder(setter(strip_option, into), default)]
    pub(super) description: Option<String>,
    #[builder(setter(strip_option), default)]
    pub(super) params: Option<Vec<ParamDefinition>>,
    pub(super) tx_input: TxInput,
    pub(super) entries: Vec<EntryInput>,
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

/// Contains the transaction-level details needed to create a `Transaction`.
#[derive(Clone, Serialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct TxInput {
    #[builder(setter(into))]
    effective: String,
    #[builder(setter(into))]
    journal_id: String,
    #[builder(setter(strip_option, into), default)]
    correlation_id: Option<String>,
    #[builder(setter(strip_option, into), default)]
    external_id: Option<String>,
    #[builder(setter(strip_option, into), default)]
    description: Option<String>,
    #[builder(setter(strip_option, into), default)]
    metadata: Option<String>,
}

impl TxInput {
    pub fn builder() -> TxInputBuilder {
        TxInputBuilder::default()
    }
}

impl TxInputBuilder {
    fn validate(&self) -> Result<(), String> {
        validate_expression(
            self.effective
                .as_ref()
                .expect("Mandatory field 'effective' not set"),
        )?;
        validate_expression(
            self.journal_id
                .as_ref()
                .expect("Mandatory field 'journal_id' not set"),
        )?;
        validate_optional_expression(&self.correlation_id)?;
        validate_optional_expression(&self.external_id)?;
        validate_optional_expression(&self.description)?;
        validate_optional_expression(&self.metadata)
    }
}

/// Contains the details for each accounting entry in a `Transaction`.
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

impl EntryInput {
    pub fn builder() -> EntryInputBuilder {
        EntryInputBuilder::default()
    }
}
impl EntryInputBuilder {
    fn validate(&self) -> Result<(), String> {
        validate_expression(
            self.entry_type
                .as_ref()
                .expect("Mandatory field 'entry_type' not set"),
        )?;
        validate_expression(
            self.account_id
                .as_ref()
                .expect("Mandatory field 'account_id' not set"),
        )?;
        validate_expression(
            self.layer
                .as_ref()
                .expect("Mandatory field 'layer' not set"),
        )?;
        validate_expression(
            self.direction
                .as_ref()
                .expect("Mandatory field 'direction' not set"),
        )?;
        validate_expression(
            self.units
                .as_ref()
                .expect("Mandatory field 'units' not set"),
        )?;
        validate_expression(
            self.currency
                .as_ref()
                .expect("Mandatory field 'currency' not set"),
        )?;
        validate_optional_expression(&self.description)
    }
}

fn validate_expression(expr: &str) -> Result<(), String> {
    CelExpression::try_from(expr).map_err(|e| e.to_string())?;
    Ok(())
}
fn validate_optional_expression(expr: &Option<Option<String>>) -> Result<(), String> {
    if let Some(Some(expr)) = expr.as_ref() {
        CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
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
        let entries = vec![EntryInput::builder()
            .entry_type("'TEST_DR'")
            .account_id("param.recipient")
            .layer("'Settled'")
            .direction("'Settled'")
            .units("1290")
            .currency("'BTC'")
            .build()
            .unwrap()];
        let new_journal = NewTxTemplate::builder()
            .id(Uuid::new_v4())
            .code("CODE")
            .tx_input(
                TxInput::builder()
                    .effective("date('2022-11-01')")
                    .journal_id(format!("'{journal_id}'"))
                    .build()
                    .unwrap(),
            )
            .entries(entries)
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
