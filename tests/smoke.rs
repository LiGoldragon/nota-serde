//! Smoke test verifying the re-export façade over nota-serde-core works.
//!
//! The real test battery lives in nota-serde-core/tests/. Here we only
//! confirm that `to_string` / `from_str` / `Error` / `Serializer` /
//! `Deserializer` are reachable via `nota_serde::` and round-trip a
//! realistic value.

use serde::{Deserialize, Serialize};

#[test]
fn reexported_api_roundtrips() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Config {
        name: String,
        port: u16,
        flags: Vec<String>,
    }

    let c = Config {
        name: "server".into(),
        port: 8080,
        flags: vec!["debug".into(), "verbose".into()],
    };
    let text = nota_serde::to_string(&c).expect("serialize");
    assert_eq!(text, "(Config server 8080 <debug verbose>)");
    let back: Config = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, c);
}

#[test]
fn error_type_reachable() {
    // Confirm the re-exported Error type works at the façade.
    let result: nota_serde::Result<i8> = nota_serde::from_str("300");
    assert!(result.is_err());
}
