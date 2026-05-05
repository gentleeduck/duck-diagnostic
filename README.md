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
  <a href="./CHANGELOG.md">Changelog</a> -
  <a href="./CONTRIBUTING.md">Contributing</a> -
  <a href="https://crates.io/crates/duck-diag">crates.io</a> -
  <a href="https://docs.rs/duck-diag">docs.rs</a>
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

## Modules

| Path | Role |
| --- | --- |
| `src/diagnostic.rs` | `Diagnostic`, `Label`, `Span`, `Severity`, `Suggestion`, `Applicability` |
| `src/formatter.rs` | terminal formatter with caret rendering, color, tab/Unicode-width handling |
| `src/smart.rs` | multi-file smart printer |
| `src/compact.rs` | source-less compact rendering + style helpers |
| `src/json.rs` | stable JSON schema (feature `json`) |
| `src/style.rs` | `RenderOptions`, color toggles, line width clamps |
| `src/macros.rs` | `diag!` macro |

## Examples

Runnable demos under [`examples/`](examples). Each one targets a
different domain (compiler, SQL, REST, config linter).

```sh
cargo run --example demo
cargo run --example sql_engine
```

See [`examples/README.md`](examples/README.md) for the full catalog.

## Build

```sh
cargo build --release
cargo test  --workspace
```

## Docs

- [crates.io](https://crates.io/crates/duck-diag) -
  [docs.rs](https://docs.rs/duck-diag) -
  [duck-ui website](https://github.com/gentleeduck/duck-ui)

## Used by

| Crate | How |
| --- | --- |
| [`@gentleduck/md` (dmc)](https://github.com/gentleeduck/duck-mc) | every layer's diagnostic engine |
| your tool here | open a PR against this README |

## Contributing

PR checklist + style notes in [`CONTRIBUTING.md`](CONTRIBUTING.md).
Security: [`SECURITY.md`](SECURITY.md). Behaviour: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

MIT. See [`LICENSE`](LICENSE).
