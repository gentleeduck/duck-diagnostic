//! Example: using duck-diag for a config file linter (YAML/TOML/JSON).

use duck_diag::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ConfigError {
  InvalidSyntax,
  MissingRequiredField,
  InvalidValue,
  DuplicateKey,
  // warnings
  DeprecatedField,
  UnknownField,
}

impl DiagnosticCode for ConfigError {
  fn code(&self) -> &str {
    match self {
      Self::InvalidSyntax => "CFG001",
      Self::MissingRequiredField => "CFG002",
      Self::InvalidValue => "CFG003",
      Self::DuplicateKey => "CFG004",
      Self::DeprecatedField => "CFG-W01",
      Self::UnknownField => "CFG-W02",
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Self::DeprecatedField | Self::UnknownField => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

fn main() {
  let config = r#"[package]
name = "my-app"
version = "1.0"
edition = "2018"
authors = ["me"]
authors = ["you"]
license = 42"#;

  let mut engine = DiagnosticEngine::<ConfigError>::new();

  engine.emit(
    Diagnostic::new(ConfigError::DuplicateKey, "duplicate key `authors`")
      .with_label(Label::primary(
        Span::new("Cargo.toml", 6, 0, 7),
        Some("second definition here".into()),
      ))
      .with_label(Label::secondary(
        Span::new("Cargo.toml", 5, 0, 7),
        Some("first defined here".into()),
      ))
      .with_help("remove one of the duplicate entries"),
  );

  engine.emit(
    Diagnostic::new(ConfigError::InvalidValue, "expected string for `license`, found integer")
      .with_label(Label::primary(
        Span::new("Cargo.toml", 7, 10, 2),
        Some("expected a string like \"MIT\"".into()),
      )),
  );

  engine.emit(
    Diagnostic::new(ConfigError::DeprecatedField, "`edition = \"2018\"` is outdated").with_label(
      Label::primary(
        Span::new("Cargo.toml", 4, 0, 18),
        Some("consider updating to \"2021\" or \"2024\"".into()),
      ),
    ),
  );

  engine.print_all(config);
}
