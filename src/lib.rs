//! Generic diagnostic engine for tools that need rich, rustc-style error
//! output. Plug in your own error code enum, attach spans + labels +
//! suggestions, and render in pretty (color), plain, or JSON modes.
//!
//! See `examples/` for end-to-end demos.

mod diagnostic;
mod formatter;
#[cfg(feature = "json")]
mod json;
mod macros;
mod utils;

pub use diagnostic::*;
pub use formatter::{DiagnosticFormatter, RenderOptions, SourceCache};

use crate::utils::*;
use colored::*;

#[derive(Debug)]
pub struct DiagnosticEngine<C: DiagnosticCode> {
  diagnostics: Vec<Diagnostic<C>>,
  bug_count: usize,
  error_count: usize,
  warning_count: usize,
  help_count: usize,
  note_count: usize,
}

impl<C: DiagnosticCode> Default for DiagnosticEngine<C> {
  fn default() -> Self {
    Self {
      diagnostics: Vec::new(),
      bug_count: 0,
      error_count: 0,
      warning_count: 0,
      help_count: 0,
      note_count: 0,
    }
  }
}

impl<C: DiagnosticCode> DiagnosticEngine<C> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn clear(&mut self) {
    self.diagnostics.clear();
    self.bug_count = 0;
    self.error_count = 0;
    self.warning_count = 0;
    self.help_count = 0;
    self.note_count = 0;
  }

  pub fn emit(&mut self, diagnostic: Diagnostic<C>) {
    match diagnostic.severity {
      Severity::Bug => self.bug_count += 1,
      Severity::Error => self.error_count += 1,
      Severity::Warning => self.warning_count += 1,
      Severity::Help => self.help_count += 1,
      Severity::Note => self.note_count += 1,
    }
    self.diagnostics.push(diagnostic);
  }

  pub fn emit_errors(&mut self, errors: Vec<Diagnostic<C>>) {
    for d in errors {
      self.emit(d);
    }
  }

  pub fn emit_warnings(&mut self, warnings: Vec<Diagnostic<C>>) {
    for d in warnings {
      self.emit(d);
    }
  }

  pub fn emit_helps(&mut self, helps: Vec<Diagnostic<C>>) {
    for d in helps {
      self.emit(d);
    }
  }

  pub fn emit_notes(&mut self, notes: Vec<Diagnostic<C>>) {
    for d in notes {
      self.emit(d);
    }
  }

  pub fn extend(&mut self, other: DiagnosticEngine<C>) {
    self.diagnostics.extend(other.diagnostics);
    self.bug_count += other.bug_count;
    self.error_count += other.error_count;
    self.warning_count += other.warning_count;
    self.help_count += other.help_count;
    self.note_count += other.note_count;
  }

  pub fn print_all(&self, source_code: &str) {
    let cache = SourceCache::new(source_code);
    for d in &self.diagnostics {
      let f = DiagnosticFormatter::with_cache(d, &cache);
      print!("{}", f.format());
    }
    let summary = self.format_summary();
    if !summary.is_empty() {
      println!("\n{}", summary);
    }
  }

  pub fn format_all(&self, source_code: &str) -> String {
    self.format_all_with(source_code, RenderOptions::default())
  }

  pub fn format_all_plain(&self, source_code: &str) -> String {
    let opts = RenderOptions { color: false, ..Default::default() };
    self.format_all_with(source_code, opts)
  }

  pub fn format_all_with(&self, source_code: &str, options: RenderOptions) -> String {
    let cache = SourceCache::new(source_code);
    let mut out = String::new();
    for d in &self.diagnostics {
      let f = DiagnosticFormatter::with_cache(d, &cache).with_options(options);
      out.push_str(&f.format());
    }
    if options.color {
      out.push_str(&self.format_summary());
    } else {
      out.push_str(&self.format_summary_plain());
    }
    out
  }

  fn format_summary(&self) -> String {
    if self.error_count == 0 && self.warning_count == 0 && self.bug_count == 0 {
      return String::new();
    }
    if self.has_errors() || self.bug_count > 0 {
      let total_errors = self.error_count + self.bug_count;
      let warn_part = if self.warning_count > 0 {
        format!(
          "; {} {} emitted",
          self.warning_count.to_string().yellow().bold(),
          pluralize("warning", self.warning_count)
        )
      } else {
        String::new()
      };
      format!(
        "{}: could not compile due to {} previous {}{}",
        "error".red().bold(),
        total_errors.to_string().red().bold(),
        pluralize("error", total_errors),
        warn_part
      )
    } else {
      format!(
        "{}: {} {} emitted",
        "warning".yellow().bold(),
        self.warning_count.to_string().yellow().bold(),
        pluralize("warning", self.warning_count)
      )
    }
  }

  fn format_summary_plain(&self) -> String {
    if self.error_count == 0 && self.warning_count == 0 && self.bug_count == 0 {
      return String::new();
    }
    if self.has_errors() || self.bug_count > 0 {
      let total_errors = self.error_count + self.bug_count;
      let warn_part = if self.warning_count > 0 {
        format!("; {} {} emitted", self.warning_count, pluralize("warning", self.warning_count))
      } else {
        String::new()
      };
      format!(
        "error: could not compile due to {} previous {}{}",
        total_errors,
        pluralize("error", total_errors),
        warn_part
      )
    } else {
      format!(
        "warning: {} {} emitted",
        self.warning_count,
        pluralize("warning", self.warning_count)
      )
    }
  }

  // getters

  pub fn get_diagnostics(&self) -> &[Diagnostic<C>] {
    &self.diagnostics
  }

  pub fn iter(&self) -> std::slice::Iter<'_, Diagnostic<C>> {
    self.diagnostics.iter()
  }

  pub fn get_errors(&self) -> Vec<&Diagnostic<C>> {
    self.diagnostics.iter().filter(|d| d.severity == Severity::Error).collect()
  }

  pub fn get_warnings(&self) -> Vec<&Diagnostic<C>> {
    self.diagnostics.iter().filter(|d| d.severity == Severity::Warning).collect()
  }

  pub fn get_notes(&self) -> Vec<&Diagnostic<C>> {
    self.diagnostics.iter().filter(|d| d.severity == Severity::Note).collect()
  }

  pub fn get_helps(&self) -> Vec<&Diagnostic<C>> {
    self.diagnostics.iter().filter(|d| d.severity == Severity::Help).collect()
  }

  pub fn get_bugs(&self) -> Vec<&Diagnostic<C>> {
    self.diagnostics.iter().filter(|d| d.severity == Severity::Bug).collect()
  }

  pub fn is_empty(&self) -> bool {
    self.diagnostics.is_empty()
  }

  pub fn len(&self) -> usize {
    self.diagnostics.len()
  }

  pub fn has_errors(&self) -> bool {
    self.error_count > 0
  }

  pub fn has_warnings(&self) -> bool {
    self.warning_count > 0
  }

  pub fn has_helps(&self) -> bool {
    self.help_count > 0
  }

  pub fn has_notes(&self) -> bool {
    self.note_count > 0
  }

  pub fn has_bugs(&self) -> bool {
    self.bug_count > 0
  }

  pub fn bug_count(&self) -> usize {
    self.bug_count
  }

  pub fn error_count(&self) -> usize {
    self.error_count
  }

  pub fn warning_count(&self) -> usize {
    self.warning_count
  }

  pub fn help_count(&self) -> usize {
    self.help_count
  }

  pub fn note_count(&self) -> usize {
    self.note_count
  }
}

#[cfg(feature = "json")]
impl<C: DiagnosticCode + serde::Serialize> DiagnosticEngine<C> {
  /// Render every diagnostic as a JSON array. Schema is stable: see
  /// [`crate::json`].
  pub fn format_all_json(&self) -> String {
    crate::json::format_all_json(&self.diagnostics)
  }
}
