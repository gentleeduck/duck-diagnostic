use duck_diagnostic::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SqlError {
  SyntaxError,
  UnknownTable,
  UnknownColumn,
  AmbiguousColumn,
  TypeMismatch,
  DivisionByZero,
  FullTableScan,
  DeprecatedSyntax,
}

impl DiagnosticCode for SqlError {
  fn code(&self) -> &str {
    match self {
      Self::SyntaxError => "SQL0001",
      Self::UnknownTable => "SQL0002",
      Self::UnknownColumn => "SQL0003",
      Self::AmbiguousColumn => "SQL0004",
      Self::TypeMismatch => "SQL0005",
      Self::DivisionByZero => "SQL0006",
      Self::FullTableScan => "SQL-W001",
      Self::DeprecatedSyntax => "SQL-W002",
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Self::FullTableScan | Self::DeprecatedSyntax => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

fn main() {
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
