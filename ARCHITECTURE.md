# ARCHITECTURE — nota-serde

The public façade for nota-text serde. Consumers — anything that
wants to round-trip Rust types through nota text — depend on
this crate, not on the kernel directly.

## Role

Thin wrapper over
[nota-serde-core](https://github.com/LiGoldragon/nota-serde-core)
configured at `Dialect::Nota`. Re-exports the serde entry points
and provides nota-specific configuration knobs (formatting,
strictness flags).

## Boundaries

Owns:

- `from_str` / `to_string` entry points pinned to nota dialect.
- Display / debug formatters specific to nota text style.

Does not own:

- Tokenisation or parsing — that's nota-serde-core.
- The grammar itself — that's
  [nota](https://github.com/LiGoldragon/nota).

## Status

CANON. Stable façade.

## Cross-cutting context

- Layer 0 of the project:
  [mentci-next/docs/architecture.md §8](https://github.com/LiGoldragon/mentci-next/blob/main/docs/architecture.md)
