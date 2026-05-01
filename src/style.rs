//! Internal styling helpers. Centralizes the `if color { x.color().bold()... }`
//! branching so renderers don't repeat it at every call site.

use colored::*;

use crate::diagnostic::{LabelStyle, Severity};

/// Apply `style_fn` only when `color` is true; otherwise return the plain string.
pub(crate) fn paint<F>(s: &str, color: bool, style_fn: F) -> String
where
  F: FnOnce(&str) -> ColoredString,
{
  if color { style_fn(s).to_string() } else { s.to_string() }
}

/// Color a string by severity + label style. Defaults to cyan-bold for
/// secondary labels and non-error severities.
pub(crate) fn paint_label(sev: Severity, style: LabelStyle, s: &str, color: bool) -> String {
  if !color {
    return s.to_string();
  }
  match (sev, style) {
    (Severity::Bug | Severity::Error, LabelStyle::Primary) => s.red().bold().to_string(),
    (Severity::Warning, LabelStyle::Primary) => s.yellow().bold().to_string(),
    _ => s.cyan().bold().to_string(),
  }
}

/// `|` gutter character.
pub(crate) fn bar(color: bool) -> String {
  paint("|", color, |s| s.blue().bold())
}

/// `-->` file-location arrow.
pub(crate) fn arrow(color: bool) -> String {
  paint("-->", color, |s| s.blue().bold())
}

/// `=` separator used for note / help / suggestion lines.
pub(crate) fn eq_sep(color: bool) -> String {
  paint("=", color, |s| s.blue().bold())
}

/// `note` / `help` label painted cyan + bold.
pub(crate) fn meta_label(label: &str, color: bool) -> String {
  paint(label, color, |s| s.cyan().bold())
}

/// Severity word (`error`, `warning`, …) painted with the severity's color.
pub(crate) fn severity_word(sev: Severity, color: bool) -> String {
  let word = sev.label();
  if !color {
    return word.to_string();
  }
  match sev {
    Severity::Bug | Severity::Error => word.red().bold().to_string(),
    Severity::Warning => word.yellow().bold().to_string(),
    _ => word.cyan().bold().to_string(),
  }
}

/// Diagnostic code painted with the severity's color.
pub(crate) fn code_word(sev: Severity, code: &str, color: bool) -> String {
  if !color {
    return code.to_string();
  }
  match sev {
    Severity::Bug | Severity::Error => code.red().bold().to_string(),
    Severity::Warning => code.yellow().bold().to_string(),
    _ => code.cyan().bold().to_string(),
  }
}

/// Pluralize one of the small fixed words used in summaries.
pub(crate) fn plural(word: &str, count: usize) -> &str {
  if count == 1 {
    word
  } else {
    match word {
      "error" => "errors",
      "warning" => "warnings",
      _ => word,
    }
  }
}
