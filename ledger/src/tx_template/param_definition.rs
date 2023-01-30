use cel_interpreter::{CelContext, CelExpression, CelType, CelValue};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct ParamDefinition {
    #[builder(setter(into))]
    pub(super) name: String,
    pub(super) r#type: ParamDataType,
    #[builder(setter(strip_option, name = "default_expr", into), default)]
    pub(super) default: Option<String>,
    #[builder(setter(strip_option, into), default)]
    pub(super) description: Option<String>,
}

impl ParamDefinition {
    pub fn builder() -> ParamDefinitionBuilder {
        ParamDefinitionBuilder::default()
    }

    pub fn default_expr(&self) -> Option<CelExpression> {
        self.default
            .as_ref()
            .map(|v| v.parse().expect("Couldn't create default_expr"))
    }
}

impl ParamDefinitionBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(Some(expr)) = self.default.as_ref() {
            let expr = CelExpression::try_from(expr.as_str()).map_err(|e| e.to_string())?;
            let param_type = ParamDataType::try_from(
                &expr
                    .evaluate(&CelContext::new())
                    .map_err(|e| format!("{e}"))?,
            )?;
            let specified_type = self.r#type.as_ref().unwrap();
            if &param_type != specified_type {
                return Err(format!(
                    "Default expression type {param_type:?} does not match parameter type {specified_type:?}"
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ParamDataType {
    STRING,
    INTEGER,
    DECIMAL,
    BOOLEAN,
    UUID,
    DATE,
    TIMESTAMP,
    JSON,
}

impl TryFrom<&CelValue> for ParamDataType {
    type Error = String;

    fn try_from(value: &CelValue) -> Result<Self, Self::Error> {
        use cel_interpreter::CelType::*;
        match CelType::from(value) {
            Int => Ok(ParamDataType::INTEGER),
            String => Ok(ParamDataType::STRING),
            Map => Ok(ParamDataType::JSON),
            Date => Ok(ParamDataType::DATE),
            Uuid => Ok(ParamDataType::UUID),
            Double => Ok(ParamDataType::DECIMAL),
            _ => Err(format!("Unsupported type: {value:?}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_param_definition() {
        let definition = ParamDefinition::builder()
            .name("name")
            .r#type(ParamDataType::JSON)
            .default_expr("{'key': 'value'}")
            .build()
            .unwrap();
        assert_eq!(definition.name, "name");
    }
}
