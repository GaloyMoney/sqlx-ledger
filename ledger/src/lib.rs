//! # sqlx-ledger
//!
//! This crate builds on the sqlx crate to provide a set of primitives for
//! implementing an SQL-compatible double-entry accounting system. This system
//! is engineered specifically for dealing with money and building financial
//! products.

#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(feature = "fail-on-warnings", deny(clippy::all))]

pub mod account;
pub mod balance;
pub mod entry;
pub mod event;
pub mod journal;
pub mod transaction;
pub mod tx_template;

mod error;
mod ledger;
mod macros;
mod primitives;

pub use error::*;
pub use ledger::*;
pub use primitives::*;
