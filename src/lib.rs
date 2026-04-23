//! nota-serde — serde Serializer + Deserializer for the nota data format.
//!
//! Implements [`serde::Serializer`] and [`serde::Deserializer`] over the
//! nota syntax: 4 delimiter pairs, 2 sigils, Pascal/camel/kebab
//! identifiers, literal forms. Any type implementing `Serialize` +
//! `Deserialize` can round-trip through nota text.
