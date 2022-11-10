use derive_builder::Builder;

use crate::primitives::*;

#[derive(Builder)]
pub struct NewAccount {
    #[builder(setter(into))]
    code: String,
    #[builder(setter(into))]
    name: String,
    #[builder(default)]
    normal_balance_type: DebitOrCredit,
    #[builder(setter(strip_option, into), default)]
    description: Option<String>,
    #[builder(default)]
    status: Status,
    #[builder(setter(custom), default)]
    metadata: Option<serde_json::Value>,
}

impl NewAccount {
    pub fn builder() -> NewAccountBuilder {
        NewAccountBuilder::default()
    }
}

impl NewAccountBuilder {
    pub fn metadata<T: serde::Serialize>(
        &mut self,
        metadata: T,
    ) -> Result<&mut Self, serde_json::Error> {
        self.metadata = Some(Some(serde_json::to_value(metadata)?));
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds() {
        let new_account = NewAccount::builder()
            .code("code")
            .name("name")
            .build()
            .unwrap();
        assert_eq!(new_account.code, "code");
        assert_eq!(new_account.name, "name");
        assert_eq!(new_account.normal_balance_type, DebitOrCredit::Credit);
        assert_eq!(new_account.description, None);
        assert_eq!(new_account.status, Status::Active);
        assert_eq!(new_account.metadata, None);
    }

    #[test]
    fn fails_when_mandatory_fields_are_missing() {
        let new_account = NewAccount::builder().build();
        assert!(new_account.is_err());
    }

    #[test]
    fn accepts_metadata() {
        use serde_json::json;
        let new_account = NewAccount::builder()
            .code("code")
            .name("name")
            .metadata(json!({"foo": "bar"}))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(new_account.metadata, Some(json!({"foo": "bar"})));
    }
}
