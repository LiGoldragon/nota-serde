//! nota-serde — serde Serializer + Deserializer for the nota data format.
//!
//! Implements [`serde::Serializer`] and [`serde::Deserializer`] over nota
//! syntax: 4 delimiter pairs (`( )` records, `[ ]` / `[| |]` strings,
//! `< >` sequences), 2 sigils (`;;` line comments, `#` byte-literal
//! prefix), Pascal/camel/kebab identifiers. Canonical form:
//! source-declaration field order, sorted map keys, shortest-roundtrip
//! numbers, single-space expression separators.
//!
//! ```
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! struct Point { horizontal: f64, vertical: f64 }
//!
//! let p = Point { horizontal: 3.0, vertical: 4.0 };
//! let text = nota_serde::to_string(&p)?;
//! assert_eq!(text, "(Point horizontal=3.0 vertical=4.0)");
//! let back: Point = nota_serde::from_str(&text)?;
//! assert_eq!(back, p);
//! # Ok::<(), nota_serde::Error>(())
//! ```

mod de;
mod error;
mod lexer;
mod ser;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
