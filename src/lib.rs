mod diagnostic;
mod formatter;
mod utils;

pub use diagnostic::*;
pub use formatter::*;

use crate::utils::*;
use colored::*;

#[derive(Debug)]
pub struct DiagnosticEngine<C: DiagnosticCode> {
  diagnostics: Vec<Diagnostic<C>>,
  error_count: usize,
  warning_count: usize,
  help_count: usize,
  note_count: usize,
}

impl<C: DiagnosticCode> Default for DiagnosticEngine<C> {
  fn default() -> Self {
    Self { diagnostics: Vec::new(), error_count: 0, warning_count: 0, help_count: 0, note_count: 0 }
  }
}

impl<C: DiagnosticCode> DiagnosticEngine<C> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn clear(&mut self) {
    self.diagnostics.clear();
    self.error_count = 0;
    self.warning_count = 0;
    self.help_count = 0;
    self.note_count = 0;
  }

  pub fn emit(&mut self, diagnostic: Diagnostic<C>) {
    match diagnostic.severity {
      Severity::Error => self.error_count += 1,
      Severity::Warning => self.warning_count += 1,
      Severity::Help => self.help_count += 1,
      Severity::Note => self.note_count += 1,
    }
    self.diagnostics.push(diagnostic);
  }

  pub fn emit_errors(&mut self, errors: Vec<Diagnostic<C>>) {
    for error in errors {
      self.emit(error);
    }
  }

  pub fn emit_warnings(&mut self, warnings: Vec<Diagnostic<C>>) {
    for warning in warnings {
      self.emit(warning);
    }
  }

  pub fn emit_helps(&mut self, helps: Vec<Diagnostic<C>>) {
    for help in helps {
      self.emit(help);
    }
  }

  pub fn emit_notes(&mut self, notes: Vec<Diagnostic<C>>) {
    for note in notes {
      self.emit(note);
    }
  }

  pub fn extend(&mut self, other: DiagnosticEngine<C>) {
    self.diagnostics.extend(other.diagnostics);
    self.error_count += other.error_count;
    self.warning_count += other.warning_count;
    self.help_count += other.help_count;
    self.note_count += other.note_count;
  }

  pub fn print_all(&self, source_code: &str) {
    for diagnostic in &self.diagnostics {
      let formatter = DiagnosticFormatter::new(diagnostic, source_code);
      print!("{}", formatter.format());
    }
    self.print_summary();
  }

  pub fn format_all(&self, source_code: &str) -> String {
    let mut output = String::new();
    for diagnostic in &self.diagnostics {
      let formatter = DiagnosticFormatter::new(diagnostic, source_code);
      output.push_str(&formatter.format());
    }
    output.push_str(&self.format_summary());
    output
  }

  pub fn format_all_plain(&self, source_code: &str) -> String {
    let mut output = String::new();
    for diagnostic in &self.diagnostics {
      let formatter = DiagnosticFormatter::new(diagnostic, source_code);
      output.push_str(&formatter.format_plain());
      output.push('\n');
    }
    output.push_str(&self.format_summary_plain());
    output
  }

  fn print_summary(&self) {
    let summary = self.format_summary();
    if !summary.is_empty() {
      println!("\n{}", summary);
    }
  }

  fn format_summary(&self) -> String {
    if self.error_count == 0 && self.warning_count == 0 {
      return String::new();
    }

    if self.has_errors() {
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
        self.error_count.to_string().red().bold(),
        pluralize("error", self.error_count),
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
    if self.error_count == 0 && self.warning_count == 0 {
      return String::new();
    }

    if self.has_errors() {
      let warn_part = if self.warning_count > 0 {
        format!("; {} {} emitted", self.warning_count, pluralize("warning", self.warning_count))
      } else {
        String::new()
      };
      format!(
        "error: could not compile due to {} previous {}{}",
        self.error_count,
        pluralize("error", self.error_count),
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
