use colored::*;

use crate::diagnostic::{Diagnostic, DiagnosticCode, LabelStyle, Severity};

pub struct DiagnosticFormatter<'a, C: DiagnosticCode> {
  diagnostic: &'a Diagnostic<C>,
  source_lines: Vec<String>,
}

impl<'a, C: DiagnosticCode> DiagnosticFormatter<'a, C> {
  pub fn new(diagnostic: &'a Diagnostic<C>, source_code: &str) -> Self {
    let source_lines: Vec<String> = source_code.lines().map(|s| s.to_string()).collect();
    Self { diagnostic, source_lines }
  }

  fn severity_text(&self) -> &'static str {
    match self.diagnostic.severity {
      Severity::Error => "error",
      Severity::Warning => "warning",
      Severity::Note => "note",
      Severity::Help => "help",
    }
  }

  fn underline_char(style: LabelStyle) -> char {
    match style {
      LabelStyle::Primary => '^',
      LabelStyle::Secondary => '-',
    }
  }

  fn get_line_content(&self, line_num: usize) -> Option<&str> {
    if line_num == 0 && self.source_lines.is_empty() {
      return None;
    }
    let index = if line_num == 0 { 0 } else { line_num - 1 };
    self.source_lines.get(index).map(|s| s.as_str())
  }

  pub fn format(&self) -> String {
    let mut out = String::new();

    let (sev, code_str) = (self.severity_text(), self.diagnostic.code.code());
    let header = match self.diagnostic.severity {
      Severity::Error => {
        format!("{}: [{}]: {}", sev.red().bold(), code_str.red().bold(), self.diagnostic.message)
      },
      Severity::Warning => format!(
        "{}: [{}]: {}",
        sev.yellow().bold(),
        code_str.yellow().bold(),
        self.diagnostic.message
      ),
      _ => {
        format!("{}: [{}]: {}", sev.cyan().bold(), code_str.cyan().bold(), self.diagnostic.message)
      },
    };
    out.push_str(&header);
    out.push('\n');

    if let Some(primary) = self.diagnostic.labels.first() {
      out.push_str(&format!(
        "  {} {}:{}:{}\n",
        "-->".blue().bold(),
        primary.span.file.white().bold(),
        primary.span.line.to_string().white().bold(),
        primary.span.column.to_string().white().bold(),
      ));

      out.push_str(&format!("   {}\n", "|".blue().bold()));

      if let Some(line_content) = self.get_line_content(primary.span.line) {
        let line_num = primary.span.line;

        out.push_str(&format!(
          " {} {} {}\n",
          format!("{}", line_num).blue().bold(),
          "|".blue().bold(),
          line_content,
        ));

        for label in &self.diagnostic.labels {
          if label.span.line != line_num {
            continue;
          }
          let ch = Self::underline_char(label.style);
          let padding = " ".repeat(label.span.column);
          let underline = ch.to_string().repeat(label.span.length);

          let colored_ul = match (self.diagnostic.severity, label.style) {
            (Severity::Error, LabelStyle::Primary) => underline.red().bold(),
            (Severity::Warning, LabelStyle::Primary) => underline.yellow().bold(),
            _ => underline.cyan().bold(),
          };

          if let Some(msg) = &label.message {
            let colored_msg = match (self.diagnostic.severity, label.style) {
              (Severity::Error, LabelStyle::Primary) => msg.red().bold(),
              (Severity::Warning, LabelStyle::Primary) => msg.yellow().bold(),
              _ => msg.cyan().bold(),
            };
            out.push_str(&format!(
              "   {} {}{} {}\n",
              "|".blue().bold(),
              padding,
              colored_ul,
              colored_msg
            ));
          } else {
            out.push_str(&format!("   {} {}{}\n", "|".blue().bold(), padding, colored_ul));
          }
        }
      }

      out.push_str(&format!("   {}\n", "|".blue().bold()));
    }

    for note in &self.diagnostic.notes {
      out.push_str(&format!("   {} {}: {}\n", "=".blue().bold(), "note".cyan().bold(), note));
    }
    if let Some(help) = &self.diagnostic.help {
      out.push_str(&format!("   {} {}: {}\n", "=".blue().bold(), "help".cyan().bold(), help));
    }

    out
  }

  pub fn format_plain(&self) -> String {
    let mut out = String::new();

    out.push_str(&format!(
      "{}: [{}]: {}\n",
      self.severity_text(),
      self.diagnostic.code.code(),
      self.diagnostic.message,
    ));

    if let Some(primary) = self.diagnostic.labels.first() {
      out.push_str(&format!(
        "  --> {}:{}:{}\n",
        primary.span.file, primary.span.line, primary.span.column,
      ));
      out.push_str("   |\n");

      if let Some(line_content) = self.get_line_content(primary.span.line) {
        let line_num = primary.span.line;
        out.push_str(&format!(" {:>3} | {}\n", line_num, line_content));

        for label in &self.diagnostic.labels {
          if label.span.line != line_num {
            continue;
          }
          let ch = Self::underline_char(label.style);
          let start_col = label.span.column.saturating_sub(1);
          let length = label.span.length.max(1);
          let padding = " ".repeat(start_col);
          let underline = ch.to_string().repeat(length);

          if let Some(msg) = &label.message {
            out.push_str(&format!("   | {}{} {}\n", padding, underline, msg));
          } else {
            out.push_str(&format!("   | {}{}\n", padding, underline));
          }
        }
      }

      out.push_str("   |\n");
    }

    for note in &self.diagnostic.notes {
      out.push_str(&format!("   = note: {}\n", note));
    }
    if let Some(help) = &self.diagnostic.help {
      out.push_str(&format!("   = help: {}\n", help));
    }

    out
  }
}
