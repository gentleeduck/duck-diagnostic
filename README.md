<p align="center">
  <img src="./public/logo.jpg" alt="gentleduck/diagnostic" width="200"/>
</p>

<h1 align="center">gentleduck/diagnostic</h1>

<p align="center">
  Generic diagnostic engine for Rust. Drop it into compilers, linters, SQL engines, config validators, whatever needs nice error output.
</p>

<p align="center">
  <a href="https://crates.io/crates/duck-diagnostic"><img src="https://img.shields.io/crates/v/duck-diagnostic.svg" alt="crates.io"></a>
  <a href="https://docs.rs/duck-diagnostic"><img src="https://docs.rs/duck-diagnostic/badge.svg" alt="docs.rs"></a>
  <a href="./LICENSE"><img src="https://img.shields.io/crates/l/duck-diagnostic.svg" alt="MIT"></a>
</p>

## Quick start

```toml
[dependencies]
duck-diagnostic = "0.1"
```

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

## How it works

You define an enum with your error codes and implement `DiagnosticCode` on it. That's it. The engine handles collecting, counting, and rendering.

```
DiagnosticEngine<C>       collects diagnostics, tracks counts, renders output
  Diagnostic<C>           single error/warning with labels, notes, help
    C: DiagnosticCode     your enum
    Label                 points at source code (span + message + style)
      Span                file + line + column + length
    notes: Vec<String>
    help: Option<String>
```

## DiagnosticCode trait

```rust
pub trait DiagnosticCode: fmt::Debug + Clone {
    fn code(&self) -> &str;
    fn severity(&self) -> Severity;
}
```

## API

### Span

```rust
Span::new(file, line, column, length)
```

Just a location. Not tied to any lexer or parser.

### Label

```rust
Label::primary(span, message)    // ^^^^ main error site
Label::secondary(span, message)  // ---- related context
```

### Diagnostic

```rust
Diagnostic::new(code, message)
    .with_label(label)
    .with_note(note)
    .with_help(help)
```

Builder methods take `impl Into<String>`, so both `&str` and `String` work.

### DiagnosticEngine

```rust
let mut engine = DiagnosticEngine::<MyError>::new();

engine.emit(diagnostic);
engine.emit_errors(vec![...]);          // batch emit
engine.emit_warnings(vec![...]);
engine.extend(other_engine);           // merge two engines

engine.has_errors();
engine.has_warnings();
engine.error_count();
engine.warning_count();

engine.print_all(source_code);          // colored terminal output
engine.format_all(source_code);         // colored string (no print)
engine.format_all_plain(source_code);   // plain text for logs/CI

engine.get_diagnostics();               // &[Diagnostic<C>]
engine.get_errors();                    // Vec<&Diagnostic<C>>
engine.get_warnings();
engine.len();
engine.is_empty();
engine.clear();
```

## Examples

**Compiler** - scanner/parser/semantic errors: [`examples/compiler.rs`](examples/compiler.rs)

**SQL engine** - unknown columns, division by zero, missing indexes: [`examples/sql_engine.rs`](examples/sql_engine.rs)

**Config linter** - duplicate keys, invalid values, deprecated fields: [`examples/config_linter.rs`](examples/config_linter.rs)

**API validator** - missing fields, bad formats, deprecated endpoints: [`examples/api_validator.rs`](examples/api_validator.rs)

**All at once**: `cargo run --example demo`

```sh
cargo run --example compiler
cargo run --example sql_engine
cargo run --example config_linter
cargo run --example api_validator
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## Security

See [SECURITY.md](./SECURITY.md) for reporting vulnerabilities.

## License

[MIT](./LICENSE) - Copyright (c) 2024 @gentleduck
