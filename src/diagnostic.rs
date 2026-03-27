use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
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
/// }
/// ```
pub trait DiagnosticCode: fmt::Debug + Clone {
  fn code(&self) -> &str;
  fn severity(&self) -> Severity;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
  pub file: String,
  pub line: usize,
  pub column: usize,
  pub length: usize,
}

impl Span {
  pub fn new(file: impl Into<String>, line: usize, column: usize, length: usize) -> Self {
    Self { file: file.into(), line, column, length }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStyle {
  Primary,
  Secondary,
}

#[derive(Debug, Clone)]
pub struct Label {
  pub span: Span,
  pub message: Option<String>,
  pub style: LabelStyle,
}

impl Label {
  pub fn primary(span: Span, message: impl Into<Option<String>>) -> Self {
    Self { span, message: message.into(), style: LabelStyle::Primary }
  }

  pub fn secondary(span: Span, message: impl Into<Option<String>>) -> Self {
    Self { span, message: message.into(), style: LabelStyle::Secondary }
  }
}

#[derive(Debug, Clone)]
pub struct Diagnostic<C: DiagnosticCode> {
  pub code: C,
  pub severity: Severity,
  pub message: String,
  pub labels: Vec<Label>,
  pub notes: Vec<String>,
  pub help: Option<String>,
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
}
