use chrono::{DateTime, Utc};
use derive_builder::Builder;

use crate::primitives::*;

/// Representation of a ledger journal entity.
pub struct Journal {
    pub id: AccountId,
    pub name: String,
    pub description: Option<String>,
    pub status: Status,
    pub version: u32,
    pub modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Representation of a new ledger journal entity
/// with required/optional properties and a builder.
#[derive(Debug, Builder)]
pub struct NewJournal {
    #[builder(setter(into))]
    pub id: JournalId,
    #[builder(setter(into))]
    pub(super) name: String,
    #[builder(setter(strip_option, into), default)]
    pub(super) description: Option<String>,
    #[builder(default)]
    pub(super) status: Status,
}

impl NewJournal {
    pub fn builder() -> NewJournalBuilder {
        let mut builder = NewJournalBuilder::default();
        builder.id(JournalId::new());
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds() {
        let new_journal = NewJournal::builder().name("name").build().unwrap();
        assert_eq!(new_journal.name, "name");
        assert_eq!(new_journal.description, None);
        assert_eq!(new_journal.status, Status::Active);
    }

    #[test]
    fn fails_when_mandatory_fields_are_missing() {
        let new_account = NewJournal::builder().build();
        assert!(new_account.is_err());
    }
}
