use duck_diagnostic::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TestCode {
  SyntaxError,
  TypeMismatch,
  UnusedVar,
}

impl DiagnosticCode for TestCode {
  fn code(&self) -> &str {
    match self {
      Self::SyntaxError => "E0001",
      Self::TypeMismatch => "E0002",
      Self::UnusedVar => "W0001",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::UnusedVar => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

#[test]
fn engine_counts_errors_and_warnings() {
  let source = "let x = 1;\nlet y = 2;\nlet z = 3;";
  let mut engine = DiagnosticEngine::<TestCode>::new();

  engine.emit(
    Diagnostic::new(TestCode::SyntaxError, "bad syntax")
      .with_label(Label::primary(Span::new("test.rs", 1, 4, 1), Some("here".into()))),
  );
  engine.emit(
    Diagnostic::new(TestCode::UnusedVar, "unused variable `y`")
      .with_label(Label::primary(Span::new("test.rs", 2, 4, 1), Some("never read".into()))),
  );
  engine.emit(
    Diagnostic::new(TestCode::TypeMismatch, "expected int, found string")
      .with_label(Label::primary(Span::new("test.rs", 3, 8, 1), Some("wrong type".into()))),
  );

  engine.print_all(source);

  assert_eq!(engine.error_count(), 2);
  assert_eq!(engine.warning_count(), 1);
  assert!(engine.has_errors());
}

#[test]
fn engine_clear_resets_everything() {
  let mut engine = DiagnosticEngine::<TestCode>::new();
  engine.emit(Diagnostic::new(TestCode::SyntaxError, "err"));
  engine.clear();

  assert_eq!(engine.error_count(), 0);
  assert_eq!(engine.warning_count(), 0);
  assert!(!engine.has_errors());
  assert!(engine.get_diagnostics().is_empty());
}

#[test]
fn diagnostic_builder_chain() {
  let source = "let val = foo + bar;";
  let d = Diagnostic::new(TestCode::SyntaxError, "unexpected token")
    .with_label(Label::primary(
      Span::new("test.rs", 1, 10, 3),
      Some("here".into()),
    ))
    .with_label(Label::secondary(
      Span::new("test.rs", 1, 16, 3),
      Some("related".into()),
    ))
    .with_note("check your syntax")
    .with_help("did you mean `let`?");

  let formatter = DiagnosticFormatter::new(&d, source);
  print!("{}", formatter.format());

  assert_eq!(d.labels.len(), 2);
  assert_eq!(d.notes.len(), 1);
  assert!(d.help.is_some());
  assert_eq!(d.severity, Severity::Error);
}

#[test]
fn plain_format_contains_code_and_message() {
  let source = "let x = ;";
  let d = Diagnostic::new(TestCode::SyntaxError, "unexpected semicolon")
    .with_label(Label::primary(Span::new("test.rs", 1, 8, 1), None));

  let formatter = DiagnosticFormatter::new(&d, source);
  let plain = formatter.format_plain();

  println!("{}", plain);

  assert!(plain.contains("E0001"));
  assert!(plain.contains("unexpected semicolon"));
  assert!(plain.contains("test.rs:1:8"));
}

#[test]
fn colored_format_contains_code_and_message() {
  let source = "let x = ;";
  let d = Diagnostic::new(TestCode::SyntaxError, "unexpected semicolon")
    .with_label(Label::primary(Span::new("test.rs", 1, 8, 1), None));

  let formatter = DiagnosticFormatter::new(&d, source);
  let colored = formatter.format();

  print!("{}", colored);

  assert!(colored.contains("unexpected semicolon"));
}

#[test]
fn format_all_plain_includes_summary() {
  let source = "line1\nline2";
  let mut engine = DiagnosticEngine::<TestCode>::new();
  engine.emit(
    Diagnostic::new(TestCode::SyntaxError, "err1")
      .with_label(Label::primary(Span::new("test.rs", 1, 0, 5), Some("first error".into()))),
  );
  engine.emit(
    Diagnostic::new(TestCode::SyntaxError, "err2")
      .with_label(Label::primary(Span::new("test.rs", 2, 0, 5), Some("second error".into()))),
  );

  let output = engine.format_all_plain(source);
  println!("{}", output);

  assert!(output.contains("2 previous errors"));
}

#[test]
fn warning_only_summary() {
  let source = "let _unused = 42;";
  let mut engine = DiagnosticEngine::<TestCode>::new();
  engine.emit(
    Diagnostic::new(TestCode::UnusedVar, "unused variable")
      .with_label(Label::primary(
        Span::new("test.rs", 1, 4, 7),
        Some("declared but never used".into()),
      ))
      .with_help("remove or prefix with `_`"),
  );

  engine.print_all(source);

  let output = engine.format_all_plain(source);
  assert!(output.contains("1 warning emitted"));
  assert!(!engine.has_errors());
}

#[test]
fn span_new_constructor() {
  let span = Span::new("file.rs", 10, 5, 3);
  assert_eq!(span.file, "file.rs");
  assert_eq!(span.line, 10);
  assert_eq!(span.column, 5);
  assert_eq!(span.length, 3);
}

#[test]
fn label_styles() {
  let span = Span::new("f.rs", 1, 0, 1);
  let primary = Label::primary(span.clone(), Some("msg".into()));
  let secondary = Label::secondary(span, None::<String>);

  assert_eq!(primary.style, LabelStyle::Primary);
  assert!(primary.message.is_some());
  assert_eq!(secondary.style, LabelStyle::Secondary);
  assert!(secondary.message.is_none());
}

#[test]
fn multiple_labels_same_line() {
  let source = "let result = a + b * c;";
  let mut engine = DiagnosticEngine::<TestCode>::new();

  engine.emit(
    Diagnostic::new(TestCode::TypeMismatch, "mismatched types in expression")
      .with_label(Label::primary(
        Span::new("expr.rs", 1, 13, 1),
        Some("this is a string".into()),
      ))
      .with_label(Label::secondary(
        Span::new("expr.rs", 1, 17, 1),
        Some("this is an int".into()),
      ))
      .with_note("cannot add `String` and `i32`")
      .with_help("convert one side: `a.parse::<i32>()`"),
  );

  engine.print_all(source);

  assert_eq!(engine.error_count(), 1);
}

#[test]
fn error_with_notes_and_help() {
  let source = "fn foo() {\n  return 42;\n}";
  let mut engine = DiagnosticEngine::<TestCode>::new();

  engine.emit(
    Diagnostic::new(TestCode::TypeMismatch, "mismatched return type")
      .with_label(Label::primary(
        Span::new("main.rs", 2, 9, 2),
        Some("expected `()`, found `i32`".into()),
      ))
      .with_note("function `foo` has no return type annotation")
      .with_note("implicit return type is `()`")
      .with_help("add a return type: `fn foo() -> i32`"),
  );

  engine.print_all(source);

  assert_eq!(engine.error_count(), 1);
}
