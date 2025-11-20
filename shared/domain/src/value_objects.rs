//! Value objects for domain modeling

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Email value object
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

/// Money value object
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64, // stored in smallest currency unit (cents)
    pub currency: Currency,
}

/// Currency enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    KES,
}
