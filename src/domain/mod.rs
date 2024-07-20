//! This module exposes the core stateful logic of the application grouped by
//! domain. Each domain enforces all business rules and invariants, unlike the raw
//! database access modules.

pub mod articles;
pub mod users;
pub mod quotes;
pub mod rules;
