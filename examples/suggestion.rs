use duck_diag::*;

#[derive(Debug, Clone, Copy)]
enum Lint {
  PreferLet,
}

impl DiagnosticCode for Lint {
  fn code(&self) -> &str {
    "L0001"
  }
  fn severity(&self) -> Severity {
    Severity::Warning
  }
  fn url(&self) -> Option<&'static str> {
    Some("https://example.com/lints/L0001")
  }
}

fn main() {
  let source = r#"function main() {
  var x = 42;
  return x;
}"#;

  let mut engine = DiagnosticEngine::<Lint>::new();
  engine.emit(
    Diagnostic::new(Lint::PreferLet, "prefer `let` over `var`")
      .with_label(
        Label::primary(Span::new("script.js", 2, 3, 3), Some("declared here".into()))
          .with_note("`var` is function-scoped"),
      )
      .with_help("`var` declarations are hoisted; `let` has block scope")
      .with_suggestion(
        Suggestion::new(Span::new("script.js", 2, 3, 3), "let")
          .with_message("replace `var` with `let`")
          .with_applicability(Applicability::MachineApplicable),
      ),
  );

  engine.print_all(source);
}
