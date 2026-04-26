use duck_diagnostic::*;

#[derive(Debug, Clone, Copy)]
enum E {
  TypeMismatch,
}

impl DiagnosticCode for E {
  fn code(&self) -> &str {
    "E0301"
  }
  fn severity(&self) -> Severity {
    Severity::Error
  }
}

fn main() {
  let source = "const greeting: string = 1;\nconst name: number = \"hi\";\n";

  let mut engine = DiagnosticEngine::<E>::new();
  engine.emit(
    Diagnostic::new(E::TypeMismatch, "type mismatch across two declarations")
      .with_label(Label::primary(
        Span::new("a.ts", 1, 25, 1),
        Some("expected `string`, got `number`".into()),
      ))
      .with_label(Label::secondary(
        Span::new("b.ts", 1, 22, 4),
        Some("expected `number`, got `string`".into()),
      ))
      .with_help("the two locations disagree on the inferred type"),
  );

  engine.print_all(source);
}
