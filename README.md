# nota-serde

[serde](https://serde.rs) Serializer + Deserializer for the
[nota](https://github.com/LiGoldragon/nota) data format. Analogous
to `serde_json` but for nota.

## Usage

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Point { horizontal: f64, vertical: f64 }

let p = Point { horizontal: 3.0, vertical: 4.0 };
let text = nota_serde::to_string(&p)?;
let back: Point = nota_serde::from_str(&text)?;
```

## License

[License of Non-Authority](LICENSE.md).
