crate::entity_id! { AccountId }
crate::entity_id! { JournalId }
crate::entity_id! { TxTemplateId }
crate::entity_id! { CorrelationId }
crate::entity_id! { ExternalId }

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "DebitOrCredit", rename_all = "snake_case")]
pub enum DebitOrCredit {
    Debit,
    Credit,
}

impl Default for DebitOrCredit {
    fn default() -> Self {
        Self::Credit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "Status", rename_all = "snake_case")]
pub enum Status {
    Active,
}

impl Default for Status {
    fn default() -> Self {
        Self::Active
    }
}
