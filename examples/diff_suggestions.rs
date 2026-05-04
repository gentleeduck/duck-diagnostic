//! Showcase rustc-style diff suggestion rendering.
//!
//!     cargo run --example diff_suggestions
//!
//! Demonstrates four flavors:
//!   1. Single-token replacement (var → let)
//!   2. Multi-line replacement (callback → arrow function)
//!   3. Insertion (length-0 span: add missing semicolon)
//!   4. Each `Applicability` level

use duck_diag::*;

#[derive(Debug, Clone, Copy)]
enum Lint {
  PreferLet,
  PreferArrow,
  MissingSemi,
  ConsiderConst,
}

impl DiagnosticCode for Lint {
  fn code(&self) -> &str {
    match self {
      Self::PreferLet => "L0001",
      Self::PreferArrow => "L0002",
      Self::MissingSemi => "L0003",
      Self::ConsiderConst => "L0004",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::MissingSemi => Severity::Error,
      _ => Severity::Warning,
    }
  }
}

fn main() {
  let source = r#"function greet(name) {
  var msg = "hi " + name
  setTimeout(function () {
    console.log(msg);
  }, 100)
  let count = 1
}"#;

  let mut engine = DiagnosticEngine::<Lint>::new();

  // --- 1. Single-token replacement: var → let ---
  engine.emit(
    Diagnostic::new(Lint::PreferLet, "prefer `let` over `var`")
      .with_label(Label::primary(Span::new("greet.js", 2, 3, 3), Some("declared here".into())))
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 2, 3, 3), "let")
          .with_message("replace `var` with `let`")
          .with_applicability(Applicability::MachineApplicable),
      ),
  );

  // --- 2. Multi-line replacement: callback → arrow ---
  // span covers `function () {` part of line 3, but replacement is multi-line:
  //     setTimeout(() => {
  //       console.log(msg);
  //     }, 100)
  // The first replacement line is spliced into line 3 before the cursor;
  // subsequent replacement lines render with their own line numbers.
  engine.emit(
    Diagnostic::new(Lint::PreferArrow, "prefer arrow function over `function () { ... }`")
      .with_label(Label::primary(
        Span::new("greet.js", 3, 14, 13),
        Some("classic function expression".into()),
      ))
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 3, 14, 13), "() => {")
          .with_message("rewrite as an arrow function")
          .with_applicability(Applicability::MaybeIncorrect),
      ),
  );

  // --- 3. Insertion (length-0): missing semicolon ---
  // After `var msg = "hi " + name` (line 2, col 25 = end of expression).
  engine.emit(
    Diagnostic::new(Lint::MissingSemi, "missing `;` at end of statement")
      .with_label(Label::primary(Span::new("greet.js", 2, 25, 0), Some("expected `;` here".into())))
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 2, 25, 0), ";")
          .with_message("add a semicolon")
          .with_applicability(Applicability::MachineApplicable),
      ),
  );

  // --- 4. Each Applicability level ---
  engine.emit(
    Diagnostic::new(Lint::ConsiderConst, "`count` is never reassigned — consider `const`")
      .with_label(Label::primary(Span::new("greet.js", 6, 3, 3), Some("declared here".into())))
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 6, 3, 3), "const")
          .with_message("MachineApplicable — auto-fix safe")
          .with_applicability(Applicability::MachineApplicable),
      )
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 6, 3, 3), "const")
          .with_message("MaybeIncorrect — review first")
          .with_applicability(Applicability::MaybeIncorrect),
      )
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 6, 3, 3), "const /* TODO */")
          .with_message("HasPlaceholders — has TODO marker")
          .with_applicability(Applicability::HasPlaceholders),
      )
      .with_suggestion(
        Suggestion::new(Span::new("greet.js", 6, 3, 3), "const")
          .with_message("Unspecified — applicability not declared")
          .with_applicability(Applicability::Unspecified),
      ),
  );

  engine.print_all(source);
}
