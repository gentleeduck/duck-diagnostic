/// Shorthand for `Diagnostic::new(...)` with optional label/note/help/suggestion.
///
/// ```rust
/// use duck_diag::*;
///
/// #[derive(Debug, Clone, Copy)]
/// enum E { Foo }
/// impl DiagnosticCode for E {
///     fn code(&self) -> &str { "E0001" }
///     fn severity(&self) -> Severity { Severity::Error }
/// }
///
/// let span = Span::new("a.rs", 1, 1, 1);
/// let d = diag!(E::Foo, span.clone(), "boom")
///     .with_help("try again")
///     .with_label(Label::primary(span, Some("here".into())));
/// assert_eq!(d.message, "boom");
/// ```
#[macro_export]
macro_rules! diag {
  ($code:expr, $span:expr, $msg:expr) => {
    $crate::Diagnostic::new($code, $msg).with_label($crate::Label::primary($span, None))
  };
  ($code:expr, $msg:expr) => {
    $crate::Diagnostic::new($code, $msg)
  };
}
