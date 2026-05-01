<p align="center">
  <img src="./public/logo-dark.svg" alt="gentleduck/diagnostic" width="120"/>
</p>

# gentleduck/diagnostic

Generic diagnostic engine for Rust. Drops into compilers, linters, SQL engines, and config validators that need rich error output.

## Links

- Crate: https://crates.io/crates/duck-diagnostic
- Docs: https://docs.rs/duck-diagnostic
- Repository: https://github.com/gentleduck/duck-diagnostic

[![crates.io](https://img.shields.io/crates/v/duck-diagnostic.svg)](https://crates.io/crates/duck-diagnostic)
[![docs.rs](https://docs.rs/duck-diagnostic/badge.svg)](https://docs.rs/duck-diagnostic)
[![MIT](https://img.shields.io/crates/l/duck-diagnostic.svg)](./LICENSE)

## Module Matrix

| Path | Role | Status |
| --- | --- | --- |
| `src/diagnostic.rs` | `Diagnostic`, `Label`, `Span`, `Severity`, `Suggestion`, `Applicability` | Active |
| `src/formatter.rs` | Terminal formatter with caret rendering, color, tab/Unicode-width handling | Active |
| `src/smart.rs` | Multi-file smart printer | Active |
| `src/compact.rs` | Source-less compact rendering and internal style helpers | Active |
| `src/json.rs` | Stable JSON schema output (feature-gated `json`) | Active |
| `src/style.rs` | `RenderOptions`, color toggles, line width clamps | Active |
| `src/macros.rs` | `diag!` macro | Active |

## Feature Matrix

| Feature | Default | Role |
| --- | --- | --- |
| `json` | yes | Enables `serde` + `serde_json`, exposes `format_all_json()` |

## Capability Matrix

| Capability | Notes |
| --- | --- |
| Multi-file diagnostics | Labels in different files render as separate sections |
| Suggestions / fix-its | `with_suggestion(...)` plus `Applicability` levels |
| rustc-style diff suggestions | `-` original / `+` replacement, red/green, aligned gutter |
| `Severity::Bug` | ICE category, separate count and color |
| JSON output | `engine.format_all_json()`, schema stable |
| `Span::from_zero_based` | Drop-in for parsers that emit 0-based positions |
| `Label::with_note` | Span-local note rendered under the caret |
| Tab + Unicode-width aware | Carets line up under emoji, CJK, tab indents |
| `SourceCache` | Split source once, reuse across diagnostics |
| `RenderOptions` | Tab width, context lines, max line width, color toggle |
| `diag!` macro | `diag!(MyError::Foo, span, "msg")` |
| Long-line truncation | `RenderOptions::max_line_width` clamps with ellipsis |
| Error code URLs | `DiagnosticCode::url()` rendered after the code |

## Install

```toml
[dependencies]
duck-diagnostic = "0.7"
```

## Usage

```rust
use duck_diagnostic::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MyError {
    SyntaxError,
    UnusedImport,
}

impl DiagnosticCode for MyError {
    fn code(&self) -> &str {
        match self {
            Self::SyntaxError  => "E0001",
            Self::UnusedImport => "W0001",
        }
    }
    fn severity(&self) -> Severity {
        match self {
            Self::SyntaxError  => Severity::Error,
            Self::UnusedImport => Severity::Warning,
        }
    }
}

fn main() {
    let source = "let x = ;";
    let mut engine = DiagnosticEngine::<MyError>::new();

    engine.emit(
        Diagnostic::new(MyError::SyntaxError, "unexpected `;`")
            .with_label(Label::primary(
                Span::new("main.lang", 1, 8, 1),
                Some("expected expression before `;`".into()),
            ))
            .with_help("try `let x = <value>;`"),
    );

    engine.print_all(source);
}
```

Output:

```
error: [E0001]: unexpected `;`
  --> main.lang:1:8
   |
 1 | let x = ;
   |         ^ expected expression before `;`
   |
   = help: try `let x = <value>;`
```

## Architecture

```
DiagnosticEngine<C>       collects diagnostics, tracks counts, renders output
  Diagnostic<C>           single error/warning with labels, notes, help
    C: DiagnosticCode     your enum
    Label                 points at source code (span + message + style)
      Span                file + line + column + length
    notes: Vec<String>
    help: Option<String>
```

## API

### `DiagnosticCode`

```rust
pub trait DiagnosticCode: fmt::Debug + Clone {
    fn code(&self) -> &str;
    fn severity(&self) -> Severity;
    fn url(&self) -> Option<&'static str> { None }
}
```

`Severity` variants: `Bug`, `Error`, `Warning`, `Note`, `Help`.

### `Span`

```rust
Span::new(file, line, column, length)
Span::from_zero_based(file, line, column, length)
```

### `Label`

```rust
Label::primary(span, message)    // ^^^^ main site
Label::secondary(span, message)  // ---- related context
```

### `Diagnostic`

```rust
Diagnostic::new(code, message)
    .with_label(label)
    .with_note(note)
    .with_help(help)
    .with_suggestion(suggestion)
```

Builder methods take `impl Into<String>`.

### `DiagnosticEngine`

```rust
let mut engine = DiagnosticEngine::<MyError>::new();

engine.emit(diagnostic);
engine.emit_errors(vec![...]);
engine.emit_warnings(vec![...]);
engine.extend(other_engine);

engine.has_errors();
engine.has_warnings();
engine.error_count();
engine.warning_count();

engine.print_all(source);
engine.format_all(source);
engine.format_all_plain(source);
engine.format_all_json();

engine.get_diagnostics();
engine.get_errors();
engine.get_warnings();
engine.len();
engine.is_empty();
engine.clear();
```

### `RenderOptions`

```rust
let opts = RenderOptions {
    tab_width: 2,
    context_lines: 2,
    max_line_width: 120,
    color: false,
};
let s = engine.format_all_with(source, opts);
```

### `SourceCache`

```rust
let cache = SourceCache::new(source);
for d in engine.get_diagnostics() {
    let f = DiagnosticFormatter::with_cache(d, &cache);
    print!("{}", f.format());
}
```

### `Suggestion`

```rust
Diagnostic::new(MyLint::PreferLet, "use `let`")
    .with_suggestion(
        Suggestion::new(span, "let")
            .with_message("replace with `let`")
            .with_applicability(Applicability::MachineApplicable),
    );
```

### `diag!`

```rust
let d = diag!(MyError::Foo, span, "msg").with_help("try this");
```

## Examples

| Path | Role |
| --- | --- |
| [`examples/compiler.rs`](examples/compiler.rs) | Scanner / parser / semantic errors |
| [`examples/sql_engine.rs`](examples/sql_engine.rs) | Unknown columns, division by zero, missing indexes |
| [`examples/config_linter.rs`](examples/config_linter.rs) | Duplicate keys, invalid values, deprecated fields |
| [`examples/api_validator.rs`](examples/api_validator.rs) | Missing fields, bad formats, deprecated endpoints |
| [`examples/suggestion.rs`](examples/suggestion.rs) | Auto-applicable rewrites |
| [`examples/diff_suggestions.rs`](examples/diff_suggestions.rs) | rustc-style `-`/`+` rendering, multi-line, applicability |
| [`examples/multi_file.rs`](examples/multi_file.rs) | Labels across two files |
| [`examples/json_output.rs`](examples/json_output.rs) | LSP / IDE-friendly JSON |
| [`examples/showcase.rs`](examples/showcase.rs) | One of every diagnostic flavor in a fake compiler run |
| [`examples/demo.rs`](examples/demo.rs) | All examples at once |

```sh
cargo run --example compiler
cargo run --example suggestion
cargo run --example json_output
cargo run --example multi_file
cargo run --example demo
```

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) and [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).

## Security

See [`SECURITY.md`](./SECURITY.md) for reporting vulnerabilities.

## License

MIT. See [`LICENSE`](./LICENSE).
