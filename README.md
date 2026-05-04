<p align="center">
  <img src="./public/logo-dark.svg" alt="duck-diag" width="120"/>
</p>

<h1 align="center">duck-diag</h1>

<p align="center">
  Generic diagnostic engine for Rust. Drops into compilers, linters,
  SQL engines, and config validators that need rich error output.
</p>

<p align="center">
  <a href="./LICENSE">MIT</a> -
  <a href="https://crates.io/crates/duck-diag">crates.io</a> -
  <a href="https://docs.rs/duck-diag">docs.rs</a> -
  <a href="https://github.com/gentleeduck/duck-diag/issues">issues</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/duck-diag"><img src="https://img.shields.io/crates/v/duck-diag.svg" alt="crates.io"/></a>
  <a href="https://docs.rs/duck-diag"><img src="https://docs.rs/duck-diag/badge.svg" alt="docs.rs"/></a>
  <a href="./LICENSE"><img src="https://img.shields.io/crates/l/duck-diag.svg" alt="MIT"/></a>
</p>

> Renamed from `duck-diagnostic` (0.7.x). New publishing line:
> `duck-diag` 0.8+. Old crate stays on crates.io for compatibility.

---

## Install

```sh
cargo add duck-diag
```

Optional `json` feature (default on):

```toml
[dependencies]
duck-diag = { version = "0.8", features = ["json"] }
```

## Quick start

```rust
use duck_diag::{Diagnostic, Label, Severity, diag};

let d = diag!(Severity::Error, "type mismatch")
  .with_label(Label::primary(span, Some("expected i32, found String".into())))
  .with_help("convert with `as i32` or change the binding type");

d.print();      // pretty terminal output
let json = d.to_json();   // stable JSON shape (with `json` feature)
```

## Module matrix

| path | role |
| --- | --- |
| `src/diagnostic.rs` | `Diagnostic`, `Label`, `Span`, `Severity`, `Suggestion`, `Applicability` |
| `src/formatter.rs` | terminal formatter with caret rendering, color, tab/Unicode-width handling |
| `src/smart.rs` | multi-file smart printer |
| `src/compact.rs` | source-less compact rendering + style helpers |
| `src/json.rs` | stable JSON schema (feature `json`) |
| `src/style.rs` | `RenderOptions`, color toggles, line width clamps |
| `src/macros.rs` | `diag!` macro |

## Why a generic engine

Most Rust tools (compilers, linters, schema validators, SQL parsers)
want the same diagnostic shape: severity + message + labels + spans +
hints. `duck-diag` gives you that surface without pulling in
miette, ariadne, or syn. Useful when you want a tiny dep that
ships color + caret rendering without the rest of the world.

## Build

```sh
cargo build --release
cargo test  --workspace
```

## Used by

| crate | how |
| --- | --- |
| [`@gentleduck/md` (dmc)](https://github.com/gentleeduck/duck-mc) | every layer's diagnostic engine |
| your tool here | open a PR against this README |

## Contributing

PR checklist + style notes in [`CONTRIBUTING.md`](CONTRIBUTING.md).
Security: [`SECURITY.md`](SECURITY.md).
Behaviour: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

MIT. See [`LICENSE`](LICENSE).
