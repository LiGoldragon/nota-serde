//! Smoke test verifying the re-export façade over nota-serde-core works.
//!
//! The real test battery lives in nota-serde-core/tests/. Here we only
//! confirm that `to_string` / `from_str` / `Error` / `Serializer` /
//! `Deserializer` are reachable via `nota_serde::` and round-trip a
//! realistic value through the new (v3) delimiter set: `( )` records,
//! `[ ]` sequences, `" "` inline strings, `""" """` multiline strings.

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
    // Ident-shaped strings emit bare; the Vec uses `[ ]` (the new
    // sequence delimiter — was `< >` in the v2 grammar).
    assert_eq!(text, "(Config server 8080 [debug verbose])");
    let back: Config = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, c);
}

#[test]
fn error_type_reachable() {
    // Confirm the re-exported Error type works at the façade.
    let result: nota_serde::Result<i8> = nota_serde::from_str("300");
    assert!(result.is_err());
}

#[test]
fn inline_string_roundtrips() {
    // String containing a space must use the inline `" "` form (new
    // — was `[ ]` in v2).
    let v = "hello world".to_string();
    let text = nota_serde::to_string(&v).expect("serialize");
    assert_eq!(text, "\"hello world\"");
    let back: String = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, v);
}

#[test]
fn multiline_string_roundtrips() {
    // String with a newline forces the `""" """` form (new — was
    // `[| |]` in v2).
    let v = "line one\nline two".to_string();
    let text = nota_serde::to_string(&v).expect("serialize");
    assert!(
        text.starts_with("\"\"\""),
        "expected multiline form, got {text:?}"
    );
    let back: String = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, v);
}

#[test]
fn sequence_uses_brackets_not_angles() {
    // The v3 grammar drops `< >` for sequences; `[ ]` is the only
    // valid sequence delimiter.
    let v = vec![1i32, 2, 3];
    let text = nota_serde::to_string(&v).expect("serialize");
    assert_eq!(text, "[1 2 3]");
    let back: Vec<i32> = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, v);

    // Old-form `<1 2 3>` no longer parses.
    let result: nota_serde::Result<Vec<i32>> = nota_serde::from_str("<1 2 3>");
    assert!(result.is_err());
}

#[test]
fn old_inline_string_form_no_longer_parses() {
    // `[hello]` was the old inline-string form; in v3 it lexes as
    // a one-element sequence with one bare-ident string. So it
    // *can* still parse back into a `Vec<String>` but not into a
    // `String` directly.
    let result_string: nota_serde::Result<String> = nota_serde::from_str("[hello]");
    assert!(result_string.is_err());
    // It does parse as a Vec<String> though, since `[hello]` is now
    // a one-element sequence.
    let v: Vec<String> = nota_serde::from_str("[hello]").expect("vec parse");
    assert_eq!(v, vec!["hello".to_string()]);
}

#[test]
fn nested_record_roundtrip() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Inner {
        label: String,
        value: i32,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Outer {
        tag: String,
        inner: Inner,
    }
    let v = Outer {
        tag: "outer".into(),
        inner: Inner { label: "inside".into(), value: 7 },
    };
    let text = nota_serde::to_string(&v).expect("serialize");
    assert_eq!(text, "(Outer outer (Inner inside 7))");
    let back: Outer = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, v);
}
