//! Use [ledger.events()](crate::SqlxLedger::events()) to subscribe to events triggered by changes to the ledger.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgListener, PgPool};
use tokio::{
    sync::{
        broadcast::{self, error::RecvError},
        RwLock,
    },
    task,
};
use tracing::instrument;

use std::{collections::HashMap, sync::Arc};

use crate::{
    balance::BalanceDetails, transaction::Transaction, AccountId, JournalId, SqlxLedgerError,
};

/// Contains fields to store & manage various ledger-related `SqlxLedgerEvent` event receivers.
#[derive(Debug, Clone)]
pub struct EventSubscriber {
    buffer: usize,
    #[allow(clippy::type_complexity)]
    balance_receivers:
        Arc<RwLock<HashMap<(JournalId, AccountId), broadcast::Sender<SqlxLedgerEvent>>>>,
    journal_receivers: Arc<RwLock<HashMap<JournalId, broadcast::Sender<SqlxLedgerEvent>>>>,
    all: Arc<broadcast::Receiver<SqlxLedgerEvent>>,
}

impl EventSubscriber {
    pub(crate) async fn connect(pool: &PgPool, buffer: usize) -> Result<Self, SqlxLedgerError> {
        let mut incoming = subscribe(pool, buffer).await?;
        let all = Arc::new(incoming.resubscribe());
        let balance_receivers = Arc::new(RwLock::new(HashMap::<
            (JournalId, AccountId),
            broadcast::Sender<SqlxLedgerEvent>,
        >::new()));
        let journal_receivers = Arc::new(RwLock::new(HashMap::<
            JournalId,
            broadcast::Sender<SqlxLedgerEvent>,
        >::new()));
        let inner_balance_receivers = Arc::clone(&balance_receivers);
        let inner_journal_receivers = Arc::clone(&journal_receivers);
        tokio::spawn(async move {
            loop {
                match incoming.recv().await {
                    Ok(event) => {
                        let journal_id = event.journal_id();
                        if let Some(journal_receivers) =
                            inner_journal_receivers.read().await.get(&journal_id)
                        {
                            let _ = journal_receivers.send(event.clone());
                        }
                        if let Some(account_id) = event.account_id() {
                            let receivers = inner_balance_receivers.read().await;
                            if let Some(receiver) = receivers.get(&(journal_id, account_id)) {
                                let _ = receiver.send(event);
                            }
                        }
                    }
                    Err(RecvError::Lagged(_)) => (),
                    Err(RecvError::Closed) => {
                        tracing::warn!("Event subscriber closed");
                        break;
                    }
                }
            }
        });
        Ok(Self {
            buffer,
            balance_receivers,
            journal_receivers,
            all,
        })
    }

    pub fn all(&self) -> broadcast::Receiver<SqlxLedgerEvent> {
        self.all.resubscribe()
    }

    pub async fn account_balance(
        &self,
        journal_id: JournalId,
        account_id: AccountId,
    ) -> broadcast::Receiver<SqlxLedgerEvent> {
        let mut listeners = self.balance_receivers.write().await;
        let mut ret = None;
        let sender = listeners
            .entry((journal_id, account_id))
            .or_insert_with(|| {
                let (sender, recv) = broadcast::channel(self.buffer);
                ret = Some(recv);
                sender
            });
        ret.unwrap_or_else(|| sender.subscribe())
    }

    pub async fn journal(&self, journal_id: JournalId) -> broadcast::Receiver<SqlxLedgerEvent> {
        let mut listeners = self.journal_receivers.write().await;
        let mut ret = None;
        let sender = listeners.entry(journal_id).or_insert_with(|| {
            let (sender, recv) = broadcast::channel(self.buffer);
            ret = Some(recv);
            sender
        });
        ret.unwrap_or_else(|| sender.subscribe())
    }
}

/// Representation of a ledger event.
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "EventRaw")]
pub struct SqlxLedgerEvent {
    pub id: i64,
    pub data: SqlxLedgerEventData,
    pub r#type: SqlxLedgerEventType,
    pub recorded_at: DateTime<Utc>,
    pub span: tracing::Span,
}

impl SqlxLedgerEvent {
    pub fn journal_id(&self) -> JournalId {
        match &self.data {
            SqlxLedgerEventData::BalanceUpdated(b) => b.journal_id,
            SqlxLedgerEventData::TransactionCreated(t) => t.journal_id,
            SqlxLedgerEventData::TransactionUpdated(t) => t.journal_id,
        }
    }

    pub fn account_id(&self) -> Option<AccountId> {
        match &self.data {
            SqlxLedgerEventData::BalanceUpdated(b) => Some(b.account_id),
            _ => None,
        }
    }
}

/// Represents the different kinds of data that can be included in an `SqlxLedgerEvent` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum SqlxLedgerEventData {
    BalanceUpdated(BalanceDetails),
    TransactionCreated(Transaction),
    TransactionUpdated(Transaction),
}

/// Defines possible event types for `SqlxLedgerEvent`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SqlxLedgerEventType {
    BalanceUpdated,
    TransactionCreated,
    TransactionUpdated,
}

pub(crate) async fn subscribe(
    pool: &PgPool,
    buffer: usize,
) -> Result<broadcast::Receiver<SqlxLedgerEvent>, SqlxLedgerError> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("sqlx_ledger_events").await?;
    let (snd, recv) = broadcast::channel(buffer);
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

#[instrument(name = "sqlx_ledger.notification_received", skip(sender), err)]
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
            span: tracing::Span::current(),
        })
    }
}
