use duck_diag::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LangError {
  UnterminatedString,
  UnexpectedToken,
  TypeMismatch,
  UnusedVariable,
}

impl DiagnosticCode for LangError {
  fn code(&self) -> &str {
    match self {
      Self::UnterminatedString => "E0001",
      Self::UnexpectedToken => "E0100",
      Self::TypeMismatch => "E0201",
      Self::UnusedVariable => "W0001",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::UnusedVariable => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SqlError {
  UnknownColumn,
  DivisionByZero,
  FullTableScan,
}

impl DiagnosticCode for SqlError {
  fn code(&self) -> &str {
    match self {
      Self::UnknownColumn => "SQL0003",
      Self::DivisionByZero => "SQL0006",
      Self::FullTableScan => "SQL-W001",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::FullTableScan => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ConfigError {
  DuplicateKey,
  InvalidValue,
  DeprecatedField,
}

impl DiagnosticCode for ConfigError {
  fn code(&self) -> &str {
    match self {
      Self::DuplicateKey => "CFG004",
      Self::InvalidValue => "CFG003",
      Self::DeprecatedField => "CFG-W01",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::DeprecatedField => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

fn demo_compiler() {
  println!("[compiler]\n");

  let source = r#"fn main() {
    let name = "hello
    let x = 42;
    let result = a + b * c;
    println(name);
}"#;

  let mut engine = DiagnosticEngine::<LangError>::new();

  engine.emit(
    Diagnostic::new(LangError::UnterminatedString, "unterminated string literal")
      .with_label(Label::primary(
        Span::new("main.lang", 2, 16, 6),
        Some("string starts here but never closes".into()),
      ))
      .with_help("close the string with a matching `\"`"),
  );

  engine.emit(
    Diagnostic::new(LangError::TypeMismatch, "mismatched types in expression")
      .with_label(Label::primary(Span::new("main.lang", 4, 17, 1), Some("this is a string".into())))
      .with_label(Label::secondary(Span::new("main.lang", 4, 21, 1), Some("this is an int".into())))
      .with_note("cannot add `String` and `i32`")
      .with_help("convert one side: `a.parse::<i32>()`"),
  );

  engine.emit(
    Diagnostic::new(LangError::UnusedVariable, "unused variable `x`")
      .with_label(Label::primary(
        Span::new("main.lang", 3, 8, 1),
        Some("declared here but never used".into()),
      ))
      .with_help("prefix with `_` to silence: `_x`"),
  );

  engine.print_all(source);
}

fn demo_sql() {
  println!("\n[sql engine]\n");

  let query = r#"SELECT u.name, o.total
FROM users u
JOIN orders o ON u.id = o.user_id
WHERE u.age / 0 > 10
  AND o.status = active"#;

  let mut engine = DiagnosticEngine::<SqlError>::new();

  engine.emit(
    Diagnostic::new(SqlError::DivisionByZero, "division by zero in expression")
      .with_label(Label::primary(
        Span::new("query.sql", 4, 6, 11),
        Some("this will always fail at runtime".into()),
      ))
      .with_note("division by a literal zero is never valid"),
  );

  engine.emit(
    Diagnostic::new(SqlError::UnknownColumn, "unknown column `active`")
      .with_label(Label::primary(
        Span::new("query.sql", 5, 18, 6),
        Some("not a known column".into()),
      ))
      .with_help("did you mean the string `'active'`?"),
  );

  engine.emit(
    Diagnostic::new(SqlError::FullTableScan, "query requires a full table scan on `users`")
      .with_label(Label::primary(
        Span::new("query.sql", 2, 5, 7),
        Some("no index on `users.age`".into()),
      ))
      .with_help("consider adding an index: CREATE INDEX idx_users_age ON users(age)"),
  );

  engine.print_all(query);
}

fn demo_config() {
  println!("\n[config linter]\n");

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

fn main() {
  demo_compiler();
  demo_sql();
  demo_config();
}
