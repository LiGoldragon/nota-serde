//! nota-serde — serde Serializer + Deserializer for the nota data format.
//!
//! Thin public façade over [`nota_serde_core`]. The kernel (Lexer,
//! Token, Error, Serializer, Deserializer, and all the ser/de
//! machinery for nota's grammar) lives in that crate and is shared
//! with [nexus-serde](https://github.com/LiGoldragon/nexus-serde).
//!
//! nota grammar: 4 delimiter pairs (`( )` records, `[ ]` / `[| |]`
//! strings, `< >` sequences), 2 sigils (`;;` line comments, `#`
//! byte-literal prefix), Pascal/camel/kebab identifiers. Records are
//! positional — field identities come from the Rust schema, not the
//! text. Canonical form: source-declaration field order, sorted map
//! keys, shortest-roundtrip numbers, single-space expression
//! separators, bare identifier-shaped strings where eligible.
//!
//! ```
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! struct Point { horizontal: f64, vertical: f64 }
//!
//! let p = Point { horizontal: 3.0, vertical: 4.0 };
//! let text = nota_serde::to_string(&p)?;
//! assert_eq!(text, "(Point 3.0 4.0)");
//! let back: Point = nota_serde::from_str(&text)?;
//! assert_eq!(back, p);
//! # Ok::<(), nota_serde::Error>(())
//! ```

pub use nota_serde_core::{from_str, to_string, Deserializer, Error, Result, Serializer};
