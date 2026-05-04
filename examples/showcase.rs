//! Big showcase. Single file. Every feature.
//!
//!     cargo run --example showcase
//!
//! Renders a fake "tinyc" compiler run that emits one of every diagnostic
//! flavor against a single source file, plus a multi-file cross-reference
//! at the end. Useful for screenshots / docs.

use duck_diag::*;

#[derive(Debug, Clone, Copy)]
enum TinyC {
  UnterminatedString,
  TypeMismatch,
  UnusedVariable,
  PreferConst,
  DeprecatedApi,
  UnreachableCode,
  CrossFileMismatch,
  Ice,
}

impl DiagnosticCode for TinyC {
  fn code(&self) -> &str {
    match self {
      Self::UnterminatedString => "E0001",
      Self::TypeMismatch => "E0201",
      Self::UnusedVariable => "W0001",
      Self::PreferConst => "L0001",
      Self::DeprecatedApi => "W0042",
      Self::UnreachableCode => "W0099",
      Self::CrossFileMismatch => "E0301",
      Self::Ice => "ICE0001",
    }
  }
  fn severity(&self) -> Severity {
    match self {
      Self::UnterminatedString | Self::TypeMismatch | Self::CrossFileMismatch => Severity::Error,
      Self::Ice => Severity::Bug,
      _ => Severity::Warning,
    }
  }
  fn url(&self) -> Option<&'static str> {
    match self {
      Self::UnterminatedString => Some("https://tinyc.example/E0001"),
      Self::TypeMismatch => Some("https://tinyc.example/E0201"),
      Self::PreferConst => Some("https://tinyc.example/L0001"),
      _ => None,
    }
  }
}

fn main() {
  let source = r#"fn greet(name: string) {
  let msg = "hello, " + name
  setTimeout(function () {
    console.log(msg);
  }, 100)
  let result = a + b * "five"
  let count = 1
  return msg
  unreachable_code_here()
  let _temp = old_api(42)
}"#;

  // Build one cache and reuse it across diagnostics so we exercise
  // SourceCache + with_cache fast path.
  let cache = SourceCache::new(source);
  let mut engine = DiagnosticEngine::<TinyC>::new();

  // ─── 1. Lexer error: unterminated string with start+end labels ───
  engine.emit(
    Diagnostic::new(TinyC::UnterminatedString, "unterminated string literal")
      .with_label(
        Label::primary(Span::new("tinyc/main.tc", 2, 13, 9), Some("string opens here".into()))
          .with_note("string literals cannot span multiple lines"),
      )
      .with_label(Label::secondary(
        Span::new("tinyc/main.tc", 2, 29, 1),
        Some("expected `\"` before end of line".into()),
      ))
      .with_help("close the string with a matching `\"` or use `\"\"\"...\"\"\"` for multi-line")
      .with_suggestion(
        Suggestion::new(Span::new("tinyc/main.tc", 2, 29, 0), "\"")
          .with_message("close the string here")
          .with_applicability(Applicability::MachineApplicable),
      ),
  );

  // ─── 2. Type error with two primary labels (pointing at each operand) ───
  engine.emit(
    Diagnostic::new(TinyC::TypeMismatch, "mismatched types in arithmetic")
      .with_label(Label::primary(
        Span::new("tinyc/main.tc", 6, 16, 1),
        Some("this is a `number`".into()),
      ))
      .with_label(Label::primary(
        Span::new("tinyc/main.tc", 6, 24, 6),
        Some("this is a `string`".into()),
      ))
      .with_note("operator `*` is not defined for `number` and `string`")
      .with_help("convert one side: `a + b * \"five\".parse::<number>()?`"),
  );

  // ─── 3. Lint with rustc-style diff suggestion ───
  engine.emit(
    Diagnostic::new(TinyC::PreferConst, "`count` is never reassigned — prefer `const`")
      .with_label(Label::primary(Span::new("tinyc/main.tc", 7, 3, 3), Some("declared here".into())))
      .with_suggestion(
        Suggestion::new(Span::new("tinyc/main.tc", 7, 3, 3), "const")
          .with_message("replace `let` with `const`")
          .with_applicability(Applicability::MachineApplicable),
      ),
  );

  // ─── 4. Warning with deprecated API ───
  engine.emit(
    Diagnostic::new(TinyC::DeprecatedApi, "`old_api` is deprecated since v2.0.0")
      .with_label(
        Label::primary(Span::new("tinyc/main.tc", 10, 15, 7), Some("called here".into()))
          .with_note("use `new_api(value, options)` instead"),
      )
      .with_help("the deprecation will become a hard error in v3.0.0"),
  );

  // ─── 5. Unused-variable warning ───
  engine.emit(
    Diagnostic::new(TinyC::UnusedVariable, "unused variable `msg`")
      .with_label(Label::primary(
        Span::new("tinyc/main.tc", 2, 7, 3),
        Some("declared but never used after this scope".into()),
      ))
      .with_help("prefix with `_` to silence: `_msg`"),
  );

  // ─── 6. Unreachable-code warning ───
  engine.emit(
    Diagnostic::new(TinyC::UnreachableCode, "unreachable code")
      .with_label(Label::primary(
        Span::new("tinyc/main.tc", 9, 3, 24),
        Some("this expression is never evaluated".into()),
      ))
      .with_label(Label::secondary(
        Span::new("tinyc/main.tc", 8, 3, 10),
        Some("any code following this `return` is dead".into()),
      )),
  );

  // ─── 7. Multi-file cross-reference (callee in another file) ───
  engine.emit(
    Diagnostic::new(TinyC::CrossFileMismatch, "function signature mismatch across files")
      .with_label(Label::primary(
        Span::new("tinyc/main.tc", 3, 3, 10),
        Some("called with 1 argument here".into()),
      ))
      .with_label(Label::secondary(
        Span::new("tinyc/runtime.tc", 1, 1, 30),
        Some("but defined to take 2 arguments".into()),
      ))
      .with_help("update either the call site or the declaration to match"),
  );

  // ─── 8. Internal compiler error (Severity::Bug) ───
  engine.emit(
    Diagnostic::new(TinyC::Ice, "compiler hit `unreachable!()` while lowering AST")
      .with_label(Label::primary(
        Span::new("tinyc/main.tc", 11, 1, 1),
        Some("triggered while lowering this token".into()),
      ))
      .with_note("this is a bug in the compiler, not your code")
      .with_help("please file a bug report at https://tinyc.example/issues with the source above"),
  );

  // Render everything against the shared cache.
  for d in engine.iter() {
    let f = DiagnosticFormatter::with_cache(d, &cache);
    print!("{}", f.format());
  }

  // Engine summary line.
  println!();
  println!("{}", engine_summary(&engine));
}

fn engine_summary<C: DiagnosticCode>(e: &DiagnosticEngine<C>) -> String {
  // Quick custom one-liner. The engine's built-in summary is also available
  // via format_all_with(...), but here we print it manually so the loop above
  // can render diagnostics via a borrowed cache.
  let mut parts = Vec::new();
  if e.bug_count() > 0 {
    parts.push(format!("{} internal-error(s)", e.bug_count()));
  }
  if e.error_count() > 0 {
    parts.push(format!("{} error(s)", e.error_count()));
  }
  if e.warning_count() > 0 {
    parts.push(format!("{} warning(s)", e.warning_count()));
  }
  format!("== summary: {} ==", parts.join(", "))
}
