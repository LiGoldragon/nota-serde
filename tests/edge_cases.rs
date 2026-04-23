//! Edge-case battery for nota-serde. Exercises the surface of the
//! grammar where bugs hide: dedent corners, numeric boundaries, unicode,
//! deep nesting, map canonicalisation, and the full error surface.
//!
//! Organised by theme in module blocks — `cargo test` runs all; filter
//! with e.g. `cargo test --test edge_cases dedent::`.

use nota_serde::{from_str, to_string};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Dedent — the headline feature. Documents every corner of the algorithm
// (strip common leading whitespace from non-blank lines; drop leading and
// trailing blank lines; keep internal blank lines; tabs and spaces each
// count as 1 byte of indent).

mod dedent {
    use super::from_str;

    #[test]
    fn strips_common_4_space_indent() {
        let text = "[|\n    hello\n    world\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello\nworld");
    }

    #[test]
    fn strips_common_2_space_indent() {
        let text = "[|\n  a\n    b\n|]";
        let s: String = from_str(text).unwrap();
        // min-indent is 2; `b`'s 4-space line keeps 2 spaces after strip.
        assert_eq!(s, "a\n  b");
    }

    #[test]
    fn strips_common_tab_indent() {
        let text = "[|\n\thello\n\tworld\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello\nworld");
    }

    #[test]
    fn no_indent_keeps_content() {
        let text = "[|\nhello\nworld\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello\nworld");
    }

    #[test]
    fn single_content_line() {
        let text = "[|\n    hello\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn preserves_internal_blank_line() {
        let text = "[|\n  a\n\n  b\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "a\n\nb");
    }

    #[test]
    fn skips_leading_blank_lines() {
        let text = "[|\n\n\n  hello\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn skips_trailing_blank_lines() {
        let text = "[|\n  hello\n\n\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn all_blank_yields_empty() {
        let text = "[|\n\n  \n\t\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "");
    }

    #[test]
    fn completely_empty_yields_empty() {
        let text = "[|\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "");
    }

    #[test]
    fn min_indent_computed_from_nonblank_lines_only() {
        // Blank lines don't influence min_indent. Line 2's empty string
        // should not force min_indent to zero.
        let text = "[|\n    a\n\n    b\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "a\n\nb");
    }

    #[test]
    fn one_shorter_line_kept_as_is() {
        // First content line has 4 spaces, second has 2 → min is 2,
        // first strips to 2 leading spaces remaining.
        let text = "[|\n    x\n  y\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "  x\ny");
    }

    #[test]
    fn mixed_tab_and_space_counted_equally() {
        // Current policy: each byte of leading whitespace counts 1,
        // regardless of whether it's a tab or a space. Documents
        // behavior; revisit if consumers hit this in practice.
        let text = "[|\n\t x\n  y\n|]";
        // Both lines have 2 bytes of leading whitespace → min is 2,
        // both strip fully.
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "x\ny");
    }

    #[test]
    fn crlf_line_endings_handled() {
        // Rust's str::lines() strips both \n and \r\n — content after
        // dedent should be the same as with \n alone.
        let text = "[|\r\n    hello\r\n    world\r\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "hello\nworld");
    }

    #[test]
    fn content_line_with_trailing_spaces_preserved() {
        // Trailing spaces on a content line aren't dedented, only
        // leading ones. (Though our current trim-based blank check
        // treats a space-only line as blank — see all_blank case.)
        let text = "[|\n  a   \n  b\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "a   \nb");
    }

    #[test]
    fn embedded_single_pipe_not_mistaken_for_closer() {
        // `|` alone (not `|]`) is allowed inside a multiline string.
        let text = "[|\n  a | b\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "a | b");
    }

    #[test]
    fn embedded_pipe_space_bracket_allowed() {
        // `| ]` with intervening space is not a closer.
        let text = "[|\n  a | ] b\n|]";
        let s: String = from_str(text).unwrap();
        assert_eq!(s, "a | ] b");
    }

    #[test]
    fn inline_string_with_bracket_forces_multiline() {
        // Round-trip: a value containing `]` can't use inline form,
        // so the serializer switches to `[| |]` automatically.
        let original = "a]b".to_string();
        let text = nota_serde::to_string(&original).unwrap();
        assert!(text.starts_with("[|"), "got {text:?}");
        let back: String = from_str(&text).unwrap();
        assert_eq!(back, original);
    }
}

// ---------------------------------------------------------------------------
// Numeric edge cases.

mod numbers {
    use super::*;

    #[test]
    fn i8_min_max() {
        let a: i8 = i8::MAX;
        let b: i8 = i8::MIN;
        assert_eq!(from_str::<i8>(&to_string(&a).unwrap()).unwrap(), a);
        assert_eq!(from_str::<i8>(&to_string(&b).unwrap()).unwrap(), b);
    }

    #[test]
    fn i64_min_max() {
        let a: i64 = i64::MAX;
        let b: i64 = i64::MIN;
        assert_eq!(from_str::<i64>(&to_string(&a).unwrap()).unwrap(), a);
        assert_eq!(from_str::<i64>(&to_string(&b).unwrap()).unwrap(), b);
    }

    #[test]
    fn u128_max() {
        let a: u128 = u128::MAX;
        assert_eq!(from_str::<u128>(&to_string(&a).unwrap()).unwrap(), a);
    }

    #[test]
    fn u64_max_round_trip() {
        let a: u64 = u64::MAX;
        assert_eq!(from_str::<u64>(&to_string(&a).unwrap()).unwrap(), a);
    }

    #[test]
    fn u128_beyond_i128_max() {
        // Exactly i128::MAX + 1 — the boundary where the lexer falls
        // back from i128 to u128.
        let a: u128 = (i128::MAX as u128) + 1;
        assert_eq!(from_str::<u128>(&to_string(&a).unwrap()).unwrap(), a);
    }

    #[test]
    fn negative_literal_cannot_deserialize_as_u64() {
        // The lexer tokenises `-5` as Token::Int(-5); deserialize_u64
        // must reject rather than silently wrapping.
        let result: Result<u64, _> = from_str("-5");
        assert!(result.is_err());
    }

    #[test]
    fn i128_min() {
        let a: i128 = i128::MIN;
        assert_eq!(from_str::<i128>(&to_string(&a).unwrap()).unwrap(), a);
    }

    #[test]
    fn parses_hex_literal() {
        let v: i32 = from_str("0xff").unwrap();
        assert_eq!(v, 255);
    }

    #[test]
    fn parses_binary_literal() {
        let v: i32 = from_str("0b1010").unwrap();
        assert_eq!(v, 10);
    }

    #[test]
    fn parses_octal_literal() {
        let v: i32 = from_str("0o755").unwrap();
        assert_eq!(v, 493);
    }

    #[test]
    fn parses_underscored_literal() {
        let v: i32 = from_str("1_000_000").unwrap();
        assert_eq!(v, 1_000_000);
    }

    #[test]
    fn negative_integer() {
        let v: i32 = from_str("-42").unwrap();
        assert_eq!(v, -42);
    }

    #[test]
    fn integer_overflow_i8_rejected() {
        let result: Result<i8, _> = from_str("200");
        assert!(result.is_err());
    }

    #[test]
    fn float_zero_and_negative_zero_distinct_bits() {
        // Rust PartialEq treats 0.0 == -0.0 as true; to_bits distinguishes.
        let neg_zero_text = to_string(&-0.0f64).unwrap();
        assert!(
            neg_zero_text.starts_with('-'),
            "negative zero should serialize with a sign: {neg_zero_text}"
        );
        let back: f64 = from_str(&neg_zero_text).unwrap();
        assert_eq!(back.to_bits(), (-0.0f64).to_bits());
    }

    #[test]
    fn float_always_has_decimal_point() {
        // Even whole-value floats must carry `.` in canonical form
        // so they tokenize as Float not Int on re-parse.
        let text = to_string(&1.0f64).unwrap();
        assert!(text.contains('.'), "got {text:?}");
    }

    #[test]
    fn float_small_positive() {
        let v: f64 = 1e-10;
        let back: f64 = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn float_large_negative() {
        let v: f64 = -1.23e18;
        let back: f64 = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn nan_rejected_on_serialize() {
        assert!(to_string(&f64::NAN).is_err());
    }

    #[test]
    fn infinity_rejected_on_serialize() {
        assert!(to_string(&f64::INFINITY).is_err());
        assert!(to_string(&f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn subnormal_round_trip() {
        let v = f64::from_bits(1);
        let back: f64 = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back.to_bits(), v.to_bits());
    }
}

// ---------------------------------------------------------------------------
// String edge cases (outside the dedent specifics above).

mod strings {
    use super::*;

    #[test]
    fn empty_inline() {
        let back: String = from_str("[]").unwrap();
        assert_eq!(back, "");
    }

    #[test]
    fn unicode_emoji() {
        let original = "hello ✨ 🌊".to_string();
        let back: String = from_str(&to_string(&original).unwrap()).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn unicode_rtl() {
        let original = "العربية".to_string();
        let back: String = from_str(&to_string(&original).unwrap()).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn semicolon_in_string_not_comment() {
        let back: String = from_str("[a ; b]").unwrap();
        assert_eq!(back, "a ; b");
    }

    #[test]
    fn double_semicolon_in_string_not_comment() {
        let back: String = from_str("[a ;; still in string]").unwrap();
        assert_eq!(back, "a ;; still in string");
    }

    #[test]
    fn hash_in_string_not_bytes() {
        let back: String = from_str("[#abc]").unwrap();
        assert_eq!(back, "#abc");
    }

    #[test]
    fn long_string_round_trip() {
        let original: String = "x".repeat(10_000);
        let back: String = from_str(&to_string(&original).unwrap()).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn multiline_forced_by_newline() {
        let original = "line one\nline two".to_string();
        let text = to_string(&original).unwrap();
        assert!(text.starts_with("[|"), "expected multiline form, got {text:?}");
        let back: String = from_str(&text).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn string_with_pipe_close_rejected() {
        // `|]` inside a string has no escape syntax; the serializer must
        // reject rather than produce ambiguous output.
        assert!(to_string(&"contains|]closer".to_string()).is_err());
    }
}

// ---------------------------------------------------------------------------
// Deep nesting and large structures.

mod nesting {
    use super::*;

    #[test]
    fn deep_option_round_trip() {
        let v: Option<Option<Option<Option<Option<i32>>>>> = Some(Some(Some(Some(Some(42)))));
        let back = from_str::<Option<Option<Option<Option<Option<i32>>>>>>(
            &to_string(&v).unwrap(),
        )
        .unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn deep_vec_round_trip() {
        let mut v: Vec<Vec<Vec<i32>>> = vec![vec![vec![1, 2, 3]]];
        for _ in 0..5 {
            v = vec![v.into_iter().flatten().flatten().map(|x| x + 1).collect::<Vec<_>>()]
                .into_iter()
                .map(|inner| vec![inner])
                .collect();
        }
        let back: Vec<Vec<Vec<i32>>> = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn deeply_nested_record_round_trip() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Node { value: i32, next: Option<Box<Node>> }

        // 50 levels deep.
        let mut head = None;
        for i in (0..50).rev() {
            head = Some(Box::new(Node { value: i, next: head }));
        }
        let v = Node { value: -1, next: head };
        let back: Node = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn large_vec_round_trip() {
        let v: Vec<i32> = (0..1_000).collect();
        let back: Vec<i32> = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn empty_containers() {
        let a: Vec<i32> = vec![];
        let b: BTreeMap<String, i32> = BTreeMap::new();
        let a_back: Vec<i32> = from_str(&to_string(&a).unwrap()).unwrap();
        let b_back: BTreeMap<String, i32> = from_str(&to_string(&b).unwrap()).unwrap();
        assert_eq!(a_back, a);
        assert_eq!(b_back, b);
    }
}

// ---------------------------------------------------------------------------
// Map canonical-sort stability.

mod maps {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn btreemap_sort_stable() {
        let mut m = BTreeMap::new();
        m.insert("c".to_string(), 3);
        m.insert("a".to_string(), 1);
        m.insert("b".to_string(), 2);
        let text = to_string(&m).unwrap();
        assert_eq!(text, "<([a] 1) ([b] 2) ([c] 3)>");
    }

    #[test]
    fn hashmap_sort_stable_across_runs() {
        // HashMap iteration order is non-deterministic; the serializer
        // must re-sort by serialized key bytes every time.
        let mut m: HashMap<&str, i32> = HashMap::new();
        for (k, v) in [("zeta", 26), ("alpha", 1), ("mu", 12), ("beta", 2)] {
            m.insert(k, v);
        }
        let text = to_string(&m).unwrap();
        assert_eq!(text, "<([alpha] 1) ([beta] 2) ([mu] 12) ([zeta] 26)>");
    }

    #[test]
    fn integer_keyed_sort_is_lexicographic_by_bytes() {
        // Canonical order is by serialised key bytes: "1" < "10" < "2".
        // Not arithmetic — deterministic, but surprising.
        let mut m: BTreeMap<i32, &str> = BTreeMap::new();
        m.insert(1, "one");
        m.insert(10, "ten");
        m.insert(2, "two");
        let text = to_string(&m).unwrap();
        assert_eq!(text, "<(1 [one]) (10 [ten]) (2 [two])>");
    }

    #[test]
    fn map_round_trip_with_many_entries() {
        let m: BTreeMap<String, i32> = (0..200)
            .map(|i| (format!("k{i:03}"), i))
            .collect();
        let back: BTreeMap<String, i32> = from_str(&to_string(&m).unwrap()).unwrap();
        assert_eq!(back, m);
    }
}

// ---------------------------------------------------------------------------
// Byte literals.

mod bytes_ {
    use super::*;
    use serde::{de::Visitor, Deserializer};

    #[derive(Debug, PartialEq)]
    struct Bytes(Vec<u8>);

    impl<'de> Deserialize<'de> for Bytes {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct V;
            impl<'de> Visitor<'de> for V {
                type Value = Bytes;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "bytes")
                }
                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Bytes, E> { Ok(Bytes(v)) }
            }
            d.deserialize_bytes(V)
        }
    }

    #[test]
    fn empty_bytes_rejected() {
        // `#` with no hex digits is a syntax error — bytes must have
        // content. If empty-bytes ever becomes a use case, this
        // assertion can flip.
        let result: nota_serde::Result<Bytes> = from_str("#");
        assert!(result.is_err());
    }

    #[test]
    fn single_byte() {
        let back: Bytes = from_str("#ab").unwrap();
        assert_eq!(back.0, vec![0xab]);
    }

    #[test]
    fn blake3_length_64() {
        let hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let back: Bytes = from_str(&format!("#{hash}")).unwrap();
        assert_eq!(back.0.len(), 32);
    }

    #[test]
    fn odd_length_hex_rejected() {
        let result: nota_serde::Result<Bytes> = from_str("#abc");
        assert!(result.is_err());
    }

    #[test]
    fn non_hex_rejected() {
        let result: nota_serde::Result<Bytes> = from_str("#xyzw");
        assert!(result.is_err());
    }
}

// ---------------------------------------------------------------------------
// Error surface. One test per distinct failure mode.

mod errors {
    use super::*;

    #[test]
    fn unit_type_serialize_rejected() {
        assert!(to_string(&()).is_err());
    }

    #[test]
    fn multi_field_tuple_struct_ser_rejected() {
        #[derive(Serialize)]
        struct Pair(i32, i32);
        let err = to_string(&Pair(3, 4)).unwrap_err().to_string();
        assert!(err.contains("multi-field unnamed struct"), "got {err}");
    }

    #[test]
    fn multi_field_tuple_struct_de_rejected() {
        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Pair(i32, i32);
        assert!(from_str::<Pair>("(Pair 3 4)").is_err());
    }

    #[test]
    fn wrong_struct_name_rejected() {
        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Point { x: f64, y: f64 }
        assert!(from_str::<Point>("(Line 1.0 2.0)").is_err());
    }

    #[test]
    fn trailing_garbage_rejected() {
        let result: Result<i32, _> = from_str("42 extra");
        assert!(result.is_err());
    }

    #[test]
    fn unclosed_record_rejected() {
        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Point { x: f64, y: f64 }
        assert!(from_str::<Point>("(Point 1.0 2.0").is_err());
    }

    #[test]
    fn unclosed_sequence_rejected() {
        let result: Result<Vec<i32>, _> = from_str("<1 2 3");
        assert!(result.is_err());
    }

    #[test]
    fn unclosed_inline_string_rejected() {
        let result: Result<String, _> = from_str("[hello");
        assert!(result.is_err());
    }

    #[test]
    fn unclosed_multiline_string_rejected() {
        let result: Result<String, _> = from_str("[|\nhello\n");
        assert!(result.is_err());
    }

    #[test]
    fn extra_field_in_struct_rejected() {
        #[derive(Deserialize, Debug)]
        #[allow(dead_code)]
        struct Point { x: f64, y: f64 }
        // Three positional values for a two-field struct.
        assert!(from_str::<Point>("(Point 1.0 2.0 3.0)").is_err());
    }
}

// ---------------------------------------------------------------------------
// Enum surface.

mod enums {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Shape {
        Point,
        Named(String),
        Sized { w: f64, h: f64 },
    }

    #[test]
    fn unit_variant_round_trip() {
        let back: Shape = from_str(&to_string(&Shape::Point).unwrap()).unwrap();
        assert_eq!(back, Shape::Point);
    }

    #[test]
    fn newtype_variant_round_trip() {
        let v = Shape::Named("thing".into());
        let back: Shape = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn struct_variant_round_trip() {
        let v = Shape::Sized { w: 3.0, h: 4.0 };
        let back: Shape = from_str(&to_string(&v).unwrap()).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn tuple_variant_with_len_2_rejected() {
        #[derive(Serialize)]
        enum E { Pair(i32, i32) }
        assert!(to_string(&E::Pair(3, 4)).is_err());
    }

    #[test]
    fn many_unit_variants_round_trip() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Big { A, B, C, D, E, F, G, H, I, J }
        for v in [Big::A, Big::B, Big::J] {
            let back: Big = from_str(&to_string(&v).unwrap()).unwrap();
            assert_eq!(back, v);
        }
    }
}

// ---------------------------------------------------------------------------
// Forbidden serde attrs — documents what happens when a user reaches for
// features the positional-records design doesn't support.

mod forbidden_attrs {
    use super::*;

    #[test]
    fn flatten_produces_map_shape_not_record() {
        // `#[serde(flatten)]` asks serde's derive to inline the nested
        // struct's fields into the parent's field list, using map-based
        // (key-name) routing. Positional records have no key-name routing,
        // so the derive falls through to serialize_map and the result is a
        // map, not a `(TypeName …)` record — almost certainly not what the
        // user wants. The nota spec forbids this construct; this test
        // documents the current (surprising) behaviour so a regression
        // toward a "clean-looking" result gets caught.
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Inner { b: i32, c: i32 }
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Outer {
            a: i32,
            #[serde(flatten)]
            inner: Inner,
        }
        let v = Outer { a: 1, inner: Inner { b: 2, c: 3 } };
        let text = to_string(&v).unwrap();
        // Output is a map form, NOT `(Outer 1 2 3)`. Map-entry order
        // is canonicalised by serialised key bytes: [a] < [b] < [c].
        assert_eq!(text, "<([a] 1) ([b] 2) ([c] 3)>");
    }
}

// ---------------------------------------------------------------------------
// Comments are ignored between any tokens.

mod comments {
    use super::*;

    #[test]
    fn leading_comment() {
        let v: i32 = from_str(";; a comment\n42").unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn trailing_comment() {
        let v: i32 = from_str("42 ;; a comment").unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn multiple_comments_between_tokens() {
        #[derive(Deserialize, PartialEq, Debug)]
        struct Point { x: i32, y: i32 }
        let text = ";; outer\n(Point ;; type\n 1 ;; x\n 2 ;; y\n)";
        let p: Point = from_str(text).unwrap();
        assert_eq!(p, Point { x: 1, y: 2 });
    }
}
