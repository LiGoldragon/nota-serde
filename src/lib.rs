//! nota-serde — serde Serializer + Deserializer for the nota data format.
//!
//! Implements [`serde::Serializer`] and [`serde::Deserializer`] over nota
//! syntax: 4 delimiter pairs (`( )` records, `[ ]` / `[| |]` strings,
//! `< >` sequences), 2 sigils (`;;` line comments, `#` byte-literal
//! prefix), Pascal/camel/kebab identifiers. Canonical form:
//! source-declaration field order, sorted map keys, shortest-roundtrip
//! numbers, single-space expression separators.
//!
//! ```ignore
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct Point { horizontal: f64, vertical: f64 }
//!
//! let p = Point { horizontal: 3.0, vertical: 4.0 };
//! let text = nota_serde::to_string(&p)?;
//! // text == "(Point horizontal=3.0 vertical=4.0)"
//! let back: Point = nota_serde::from_str(&text)?;
//! # Ok::<(), nota_serde::Error>(())
//! ```

mod de;
mod error;
mod lexer;
mod ser;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
