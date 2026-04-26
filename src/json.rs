//! JSON output mode. Stable schema for IDE / LSP / CI consumers.

use crate::diagnostic::{Diagnostic, DiagnosticCode};
use serde::Serialize;

#[derive(Serialize)]
struct JsonDiag<'a, C: DiagnosticCode + Serialize> {
  code: &'a str,
  severity: crate::diagnostic::Severity,
  message: &'a str,
  url: Option<&'static str>,
  labels: &'a [crate::diagnostic::Label],
  notes: &'a [String],
  help: &'a Option<String>,
  suggestions: &'a [crate::diagnostic::Suggestion],
  #[serde(skip_serializing_if = "Option::is_none")]
  raw_code: Option<&'a C>,
}

pub(crate) fn format_all_json<C: DiagnosticCode + Serialize>(
  diagnostics: &[Diagnostic<C>],
) -> String {
  let view: Vec<JsonDiag<C>> = diagnostics
    .iter()
    .map(|d| JsonDiag {
      code: d.code.code(),
      severity: d.severity,
      message: &d.message,
      url: d.code.url(),
      labels: &d.labels,
      notes: &d.notes,
      help: &d.help,
      suggestions: &d.suggestions,
      raw_code: Some(&d.code),
    })
    .collect();
  serde_json::to_string_pretty(&view).unwrap_or_else(|_| "[]".to_string())
}
