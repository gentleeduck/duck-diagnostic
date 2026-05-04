use duck_diag::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LangError {
  UnterminatedString,
  InvalidCharacter,
  UnexpectedToken,
  MissingSemicolon,
  ExpectedExpression,
  UndeclaredVariable,
  TypeMismatch,
  UnusedVariable,
  UnreachableCode,
}

impl DiagnosticCode for LangError {
  fn code(&self) -> &str {
    match self {
      Self::UnterminatedString => "E0001",
      Self::InvalidCharacter => "E0002",
      Self::UnexpectedToken => "E0100",
      Self::MissingSemicolon => "E0101",
      Self::ExpectedExpression => "E0102",
      Self::UndeclaredVariable => "E0200",
      Self::TypeMismatch => "E0201",
      Self::UnusedVariable => "W0001",
      Self::UnreachableCode => "W0002",
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Self::UnusedVariable | Self::UnreachableCode => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

fn main() {
  let source = r#"fn main() {
    let name = "hello
    let x = 42;
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
    Diagnostic::new(LangError::UnusedVariable, "unused variable `x`")
      .with_label(Label::primary(
        Span::new("main.lang", 3, 8, 1),
        Some("declared here but never used".into()),
      ))
      .with_help("prefix with `_` to silence: `_x`"),
  );

  engine.print_all(source);
}
