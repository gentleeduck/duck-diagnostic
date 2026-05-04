//! Example: using duck-diag for REST API request/response validation.

use duck_diag::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ApiError {
  MissingField,
  InvalidType,
  ValueOutOfRange,
  InvalidFormat,
  AuthenticationFailed,
  // warnings
  DeprecatedEndpoint,
  SlowQuery,
}

impl DiagnosticCode for ApiError {
  fn code(&self) -> &str {
    match self {
      Self::MissingField => "API001",
      Self::InvalidType => "API002",
      Self::ValueOutOfRange => "API003",
      Self::InvalidFormat => "API004",
      Self::AuthenticationFailed => "API005",
      Self::DeprecatedEndpoint => "API-W01",
      Self::SlowQuery => "API-W02",
    }
  }

  fn severity(&self) -> Severity {
    match self {
      Self::DeprecatedEndpoint | Self::SlowQuery => Severity::Warning,
      _ => Severity::Error,
    }
  }
}

fn main() {
  let request_body = r#"{
  "name": "Alice",
  "age": -5,
  "email": "not-an-email",
  "role": "superadmin"
}"#;

  let mut engine = DiagnosticEngine::<ApiError>::new();

  engine.emit(
    Diagnostic::new(ApiError::ValueOutOfRange, "`age` must be a positive integer")
      .with_label(Label::primary(
        Span::new("POST /users", 3, 9, 2),
        Some("negative values not allowed".into()),
      ))
      .with_help("age must be between 0 and 150"),
  );

  engine.emit(
    Diagnostic::new(ApiError::InvalidFormat, "`email` is not a valid email address")
      .with_label(Label::primary(
        Span::new("POST /users", 4, 11, 14),
        Some("missing @ symbol".into()),
      ))
      .with_help("expected format: user@domain.com"),
  );

  engine.emit(
    Diagnostic::new(ApiError::DeprecatedEndpoint, "POST /users is deprecated, use POST /v2/users")
      .with_label(Label::primary(Span::new("request", 1, 0, 1), None))
      .with_note("this endpoint will be removed in v3"),
  );

  engine.print_all(request_body);
}
