use crate::error::*;
use cel_interpreter::CelValue;

crate::entity_id! { AccountId }
crate::entity_id! { JournalId }
crate::entity_id! { TransactionId }
crate::entity_id! { EntryId }
crate::entity_id! { TxTemplateId }
crate::entity_id! { CorrelationId }
crate::entity_id! { ExternalId }

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "Layer", rename_all = "snake_case")]
pub enum Layer {
    Settled,
    Pending,
    Encumbered,
}

impl TryFrom<CelValue> for Layer {
    type Error = SqlxLedgerError;

    fn try_from(val: CelValue) -> Result<Self, Self::Error> {
        match val {
            CelValue::String(v) if v.as_ref() == "SETTLED" => Ok(Layer::Settled),
            CelValue::String(v) if v.as_ref() == "PENDING" => Ok(Layer::Pending),
            CelValue::String(v) if v.as_ref() == "ENCUMBERED" => Ok(Layer::Encumbered),
            v => Err(SqlxLedgerError::UnknownLayer(v)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "DebitOrCredit", rename_all = "snake_case")]
pub enum DebitOrCredit {
    Debit,
    Credit,
}

impl TryFrom<CelValue> for DebitOrCredit {
    type Error = SqlxLedgerError;

    fn try_from(val: CelValue) -> Result<Self, Self::Error> {
        match val {
            CelValue::String(v) if v.as_ref() == "DEBIT" => Ok(DebitOrCredit::Debit),
            CelValue::String(v) if v.as_ref() == "CREDIT" => Ok(DebitOrCredit::Credit),
            v => Err(SqlxLedgerError::UnknownDebitOrCredit(v)),
        }
    }
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
