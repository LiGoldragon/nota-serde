//! nota-serde — serde Serializer for the nota data format.
//!
//! Implements [`serde::Serializer`] over nota syntax: 4 delimiter pairs
//! (`( )` records, `[ ]` / `[| |]` strings, `< >` sequences), 2 sigils
//! (`;;` line comments, `#` byte-literal prefix), Pascal/camel/kebab
//! identifiers. Canonical form: source-declaration field order, sorted
//! map keys, shortest-roundtrip numbers, single-space expression
//! separators.
//!
//! The Deserializer is not yet implemented (phase 3 per
//! `mentci-next/reports/007`).
//!
//! ```ignore
//! #[derive(serde::Serialize)]
//! struct Point { horizontal: f64, vertical: f64 }
//!
//! let p = Point { horizontal: 3.0, vertical: 4.0 };
//! let text = nota_serde::to_string(&p)?;
//! // text == "(Point horizontal=3.0 vertical=4.0)"
//! # Ok::<(), nota_serde::Error>(())
//! ```

mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
