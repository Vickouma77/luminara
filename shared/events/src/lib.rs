//! Event Definitions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base event trait
pub trait Event {
    fn event_id(&self) -> Uuid;
    fn event_type(&self) -> &str;
    fn timestamp(&self) -> DateTime<Utc>;
}

/// User created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub timestamp: DateTime<Utc>,
}

impl Event for UserCreatedEvent {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    fn event_type(&self) -> &str {
        "user.created"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

/// Account created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCreatedEvent {
    pub event_id: Uuid,
    pub account_id: Uuid,
    pub user_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl Event for AccountCreatedEvent {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    fn event_type(&self) -> &str {
        "account.created"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

/// Transaction completed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCompletedEvent {
    pub event_id: Uuid,
    pub transaction_id: Uuid,
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: i64,
    pub timestamp: DateTime<Utc>,
}

impl Event for TransactionCompletedEvent {
    fn event_id(&self) -> Uuid {
        self.event_id
    }
    fn event_type(&self) -> &str {
        "transaction.completed"
    }
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}
