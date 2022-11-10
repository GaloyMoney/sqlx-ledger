use chrono::{DateTime, Utc};

use crate::primitives::*;

pub struct Account<M> {
    pub id: AccountId,
    pub code: String,
    pub name: String,
    pub normal_balance_type: DebitOrCredit,
    pub description: Option<String>,
    pub status: Status,
    pub metadata: Option<M>,
    pub version: u32,
    pub modified_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
