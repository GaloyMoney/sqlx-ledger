use uuid::Uuid;

pub struct AccountId(Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebitOrCredit {
    Debit,
    Credit,
}

impl Default for DebitOrCredit {
    fn default() -> Self {
        Self::Credit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Active,
}

impl Default for Status {
    fn default() -> Self {
        Self::Active
    }
}
