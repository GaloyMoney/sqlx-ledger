//! A [Transaction] holds metadata and is referenced by its [Entries](crate::entry::Entry).
mod entity;
mod repo;

pub use entity::*;
pub use repo::*;
