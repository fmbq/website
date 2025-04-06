//! This module exposes the core stateful logic of the application grouped by
//! domain. Each domain enforces all business rules and invariants, unlike the raw
//! database access modules.

pub mod articles;
pub mod auth;
pub mod quotes;
pub mod rules;
pub mod hall_of_fame;
pub mod markell;
pub mod material;
pub mod finals;
pub mod user;

