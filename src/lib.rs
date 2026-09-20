//! Proposal types for the bilingual meaning language.
//!
//! The Ethos source is authoritative.  The generated module is refreshed only
//! with the supported `ethos-zero Generate` command.
pub mod generated;
pub use generated::meaning::*;

pub const ETHOS: &str = include_str!("../ethos/meaning.ethos");
pub const ETHOS_RUST: &str = include_str!("generated/meaning.rs");
