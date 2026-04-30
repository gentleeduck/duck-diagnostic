use std::{fmt, sync::Arc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Severity of a diagnostic.
///
/// `Bug` is reserved for internal compiler errors (ICEs) — anything that
/// indicates a defect in the tool itself, not the user's input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Severity {
  Bug,
  Error,
  Warning,
  Note,
  Help,
}

/// Implement this on your error enum to plug into the diagnostic system.
///
/// ```rust
/// use duck_diagnostic::{DiagnosticCode, Severity};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum MyError {
///     SyntaxError,
///     UnusedImport,
/// }
///
/// impl DiagnosticCode for MyError {
///     fn code(&self) -> &str {
///         match self {
///             Self::SyntaxError  => "E0001",
///             Self::UnusedImport => "W0001",
///         }
///     }
///     fn severity(&self) -> Severity {
///         match self {
///             Self::SyntaxError  => Severity::Error,
///             Self::UnusedImport => Severity::Warning,
///         }
///     }
///     fn url(&self) -> Option<&'static str> {
///         match self {
///             Self::SyntaxError  => Some("https://example.com/E0001"),
///             _ => None,
///         }
///     }
/// }
/// ```
pub trait DiagnosticCode: fmt::Debug + Clone {
  fn code(&self) -> &str;
  fn severity(&self) -> Severity;

  /// Optional documentation URL rendered after the code in pretty mode.
  fn url(&self) -> Option<&'static str> {
    None
  }
}

/// Source span.
///
/// **Convention:** `line` and `column` are **1-based** (matches rustc / clippy / clang).
/// If your front-end emits 0-based positions, use [`Span::from_zero_based`] to convert.
///
/// `length` is in **bytes** of the underlying source slice (not characters or columns).
/// Rendering uses [`unicode-width`](https://docs.rs/unicode-width) to compute display width.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Span {
  pub file: Arc<str>,
  /// 1-based line number.
  pub line: usize,
  /// 1-based column number.
  pub column: usize,
  /// Byte length of the spanned source slice.
  pub length: usize,
}

impl Span {
  /// Construct a 1-based span. Use this when your front-end already counts
  /// from 1 (most do).
  pub fn new(file: impl Into<Arc<str>>, line: usize, column: usize, length: usize) -> Self {
    Self { file: file.into(), line, column, length }
  }

  /// Construct a span from 0-based line + column. The crate stores 1-based
  /// internally, so this just adds 1 to each.
  ///
  /// ```
  /// use duck_diagnostic::Span;
  /// let s = Span::from_zero_based("foo.rs", 0, 0, 1);
  /// assert_eq!(s.line, 1);
  /// assert_eq!(s.column, 1);
  /// ```
  pub fn from_zero_based(
    file: impl Into<Arc<str>>,
    line: usize,
    column: usize,
    length: usize,
  ) -> Self {
    Self { file: file.into(), line: line + 1, column: column + 1, length }
  }

  /// Convenience: synthetic span used for diagnostics that don't point at any
  /// real source location (e.g. CLI flag errors).
  pub fn synthetic(file: impl Into<Arc<str>>) -> Self {
    Self { file: file.into(), line: 0, column: 0, length: 0 }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum LabelStyle {
  Primary,
  Secondary,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Label {
  pub span: Span,
  pub message: Option<String>,
  pub style: LabelStyle,
  /// Optional short note rendered immediately after the caret.
  pub note: Option<String>,
}

impl Label {
  pub fn primary(span: Span, message: impl Into<Option<String>>) -> Self {
    Self { span, message: message.into(), style: LabelStyle::Primary, note: None }
  }

  pub fn secondary(span: Span, message: impl Into<Option<String>>) -> Self {
    Self { span, message: message.into(), style: LabelStyle::Secondary, note: None }
  }

  pub fn with_note(mut self, note: impl Into<String>) -> Self {
    self.note = Some(note.into());
    self
  }
}

/// How confident the suggestion is — controls whether IDEs may auto-apply it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum Applicability {
  /// Safe to apply automatically.
  MachineApplicable,
  /// Likely correct but worth a human glance.
  MaybeIncorrect,
  /// Manual review required.
  HasPlaceholders,
  /// Don't auto-apply.
  Unspecified,
}

/// Code rewrite suggestion attached to a diagnostic.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Suggestion {
  pub span: Span,
  pub replacement: String,
  pub message: Option<String>,
  pub applicability: Applicability,
}

impl Suggestion {
  pub fn new(span: Span, replacement: impl Into<String>) -> Self {
    Self {
      span,
      replacement: replacement.into(),
      message: None,
      applicability: Applicability::Unspecified,
    }
  }

  pub fn with_message(mut self, message: impl Into<String>) -> Self {
    self.message = Some(message.into());
    self
  }

  pub fn with_applicability(mut self, app: Applicability) -> Self {
    self.applicability = app;
    self
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Diagnostic<C: DiagnosticCode> {
  pub code: C,
  pub severity: Severity,
  pub message: String,
  pub labels: Vec<Label>,
  pub notes: Vec<String>,
  pub help: Option<String>,
  pub suggestions: Vec<Suggestion>,
}

impl<C: DiagnosticCode> Diagnostic<C> {
  pub fn new(code: C, message: impl Into<String>) -> Self {
    let severity = code.severity();
    Self {
      code,
      severity,
      message: message.into(),
      labels: Vec::new(),
      notes: Vec::new(),
      help: None,
      suggestions: Vec::new(),
    }
  }

  pub fn with_label(mut self, label: Label) -> Self {
    self.labels.push(label);
    self
  }

  pub fn with_note(mut self, note: impl Into<String>) -> Self {
    self.notes.push(note.into());
    self
  }

  pub fn with_help(mut self, help: impl Into<String>) -> Self {
    self.help = Some(help.into());
    self
  }

  pub fn with_suggestion(mut self, suggestion: Suggestion) -> Self {
    self.suggestions.push(suggestion);
    self
  }

  /// Override the severity inferred from the code.
  pub fn with_severity(mut self, severity: Severity) -> Self {
    self.severity = severity;
    self
  }

  /// Primary label, if any (first label, or first `Primary`-styled label).
  pub fn primary_label(&self) -> Option<&Label> {
    self.labels.iter().find(|l| l.style == LabelStyle::Primary).or_else(|| self.labels.first())
  }
}
