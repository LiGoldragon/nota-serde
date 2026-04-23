//! Phase 5 — parse a realistic `.nota` document through nota-serde.
//!
//! Covers: nested structs, enums with mixed variant kinds, Vec of structs,
//! Option, bytes, maps, multiline strings, comments. Exercises the full
//! public API on a single document.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Project {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>,
    dependencies: Vec<Dep>,
    flags: BTreeMap<String, bool>,
    license: License,
    status: Status,
    release_notes: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Dep {
    name: String,
    version: String,
    features: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum License {
    LicenseOfNonAuthority,
    Mit,
    Apache2,
    Dual { primary: String, fallback: String },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Status {
    Alpha,
    Beta,
    Released(String),
    Archived { reason: String, at_commit: String },
}

fn sample() -> Project {
    let mut flags = BTreeMap::new();
    flags.insert("debug".to_string(), true);
    flags.insert("strict".to_string(), false);
    flags.insert("verbose".to_string(), true);

    Project {
        name: "nota-serde".into(),
        version: "0.1.0".into(),
        description: "Rust serde implementation of the nota data format.\nData-layer only; nexus adds the messaging layer.".into(),
        authors: vec!["ligoldragon".into()],
        dependencies: vec![
            Dep { name: "serde".into(), version: "1".into(), features: vec!["derive".into()] },
            Dep { name: "thiserror".into(), version: "2".into(), features: vec![] },
        ],
        flags,
        license: License::LicenseOfNonAuthority,
        status: Status::Released("2026-04-23".into()),
        release_notes: None,
    }
}

#[test]
fn roundtrip_realistic_document() {
    let doc = sample();
    let text = nota_serde::to_string(&doc).expect("serialize");

    // Spot-check a few stable substrings — canonical order of fields is
    // source-declaration order.
    assert!(text.starts_with("(Project name=[nota-serde]"));
    assert!(text.contains("version=[0.1.0]"));
    assert!(text.contains("license=LicenseOfNonAuthority"));
    assert!(text.contains("(Released [2026-04-23])"));
    assert!(text.contains("release_notes=None"));

    // Canonical map sort: debug < strict < verbose.
    let flags_pos = text.find("flags=").unwrap();
    let rest = &text[flags_pos..];
    let d = rest.find("debug").unwrap();
    let s = rest.find("strict").unwrap();
    let v = rest.find("verbose").unwrap();
    assert!(d < s && s < v, "map entries not sorted: {rest}");

    let back: Project = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, doc);
}

#[test]
fn roundtrip_with_archived_status_variant() {
    let mut doc = sample();
    doc.status = Status::Archived {
        reason: "superseded by nexus-serde".into(),
        at_commit: "abcdef".into(),
    };
    doc.release_notes = Some("see report 007".into());

    let text = nota_serde::to_string(&doc).expect("serialize");
    assert!(text.contains("(Archived reason=[superseded by nexus-serde] at_commit=[abcdef])"));
    assert!(text.contains("release_notes=[see report 007]"));

    let back: Project = nota_serde::from_str(&text).expect("deserialize");
    assert_eq!(back, doc);
}

#[test]
fn parse_hand_written_document() {
    // Mimics what a developer would actually write — with comments and
    // indentation that the parser must tolerate.
    let text = r#"
        ;; Demo project manifest
        (Project
          name=[tiny]
          version=[0.0.1]
          description=[|
            two
            lines
          |]
          authors=<[anon]>
          ;; no deps yet
          dependencies=<>
          flags=<([debug] true)>
          license=Mit
          status=Alpha
          release_notes=None)
    "#;
    let p: Project = nota_serde::from_str(text).expect("parse hand-written");
    assert_eq!(p.name, "tiny");
    assert_eq!(p.version, "0.0.1");
    assert_eq!(p.description, "two\nlines");
    assert_eq!(p.authors, vec!["anon"]);
    assert_eq!(p.dependencies.len(), 0);
    assert_eq!(p.flags.get("debug"), Some(&true));
    assert!(matches!(p.license, License::Mit));
    assert!(matches!(p.status, Status::Alpha));
    assert_eq!(p.release_notes, None);
}
