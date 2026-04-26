//! Run with `cargo run --example json_output`. Pipes a JSON array on stdout.

use duck_diagnostic::*;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
enum Code {
  Lint,
  Crash,
}

impl DiagnosticCode for Code {
  fn code(&self) -> &str {
    match self {
      Self::Lint => "L0001",
      Self::Crash => "ICE0001",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::Lint => Severity::Warning,
      Self::Crash => Severity::Bug,
    }
  }
}

fn main() {
  let mut engine = DiagnosticEngine::<Code>::new();
  engine.emit(
    Diagnostic::new(Code::Lint, "use snake_case for module names")
      .with_label(Label::primary(Span::new("lib.rs", 1, 5, 6), Some("here".into())))
      .with_help("rename to `my_module`"),
  );
  engine.emit(
    Diagnostic::new(Code::Crash, "compiler hit unreachable!()")
      .with_label(Label::primary(Span::new("compiler.rs", 42, 1, 1), None))
      .with_note("please file a bug report"),
  );

  println!("{}", engine.format_all_json());
}
