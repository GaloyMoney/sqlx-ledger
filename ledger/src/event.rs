use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, PgPool};
use tokio::{sync::broadcast, task};
use tracing::instrument;

use crate::{balance::BalanceDetails, transaction::Transaction, SqlxLedgerError};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "EventRaw")]
pub struct SqlxLedgerEvent {
    pub id: i64,
    pub data: SqlxLedgerEventData,
    pub r#type: SqlxLedgerEventType,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum SqlxLedgerEventData {
    BalanceUpdated(BalanceDetails),
    TransactionCreated(Transaction),
    TransactionUpdated(Transaction),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SqlxLedgerEventType {
    BalanceUpdated,
    TransactionCreated,
    TransactionUpdated,
}

pub(crate) async fn subscribe(
    pool: &PgPool,
) -> Result<broadcast::Receiver<SqlxLedgerEvent>, SqlxLedgerError> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("sqlx_ledger_events").await?;
    let (snd, recv) = broadcast::channel(100);
    task::spawn(async move {
        let mut num_errors = 0;
        loop {
            match listener.recv().await {
                Ok(notification) => {
                    num_errors = 0;
                    let _ = sqlx_ledger_notification_received(notification.payload(), &snd);
                }
                _ if num_errors > 0 => {
                    num_errors = 0;
                }
                _ => {
                    tokio::time::sleep(std::time::Duration::from_secs(1 << num_errors)).await;
                    num_errors += 1;
                }
            }
        }
    });
    Ok(recv)
}

#[instrument(skip(sender), err)]
fn sqlx_ledger_notification_received(
    payload: &str,
    sender: &broadcast::Sender<SqlxLedgerEvent>,
) -> Result<(), SqlxLedgerError> {
    let event: SqlxLedgerEvent = serde_json::from_str(payload)?;
    sender.send(event)?;
    Ok(())
}

#[derive(Deserialize)]
struct EventRaw {
    id: i64,
    data: serde_json::Value,
    r#type: SqlxLedgerEventType,
    recorded_at: DateTime<Utc>,
}

impl TryFrom<EventRaw> for SqlxLedgerEvent {
    type Error = serde_json::Error;

    fn try_from(value: EventRaw) -> Result<Self, Self::Error> {
        let data = match value.r#type {
            SqlxLedgerEventType::BalanceUpdated => {
                SqlxLedgerEventData::BalanceUpdated(serde_json::from_value(value.data)?)
            }
            SqlxLedgerEventType::TransactionCreated => {
                SqlxLedgerEventData::TransactionCreated(serde_json::from_value(value.data)?)
            }
            SqlxLedgerEventType::TransactionUpdated => {
                SqlxLedgerEventData::TransactionUpdated(serde_json::from_value(value.data)?)
            }
        };

        Ok(SqlxLedgerEvent {
            id: value.id,
            data,
            r#type: value.r#type,
            recorded_at: value.recorded_at,
        })
    }
}
