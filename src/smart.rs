//! Source-aware printer for multi-file engines. Picks the right rendering
//! per diagnostic without forcing the caller to thread per-file source
//! strings to the print site.
//!
//! Rule:
//! - Diagnostic primary label points at a real file we can read → render
//!   with-source (snippet + carets), via [`DiagnosticFormatter`].
//! - Otherwise (synthetic span, missing file, glob/config/IO error with
//!   no source on hand) → render compact via [`crate::format_compact`].
//!
//! Source files are read at most once per render via a small cache keyed
//! by `Arc<str>` path.

use std::collections::HashMap;
use std::io::IsTerminal;
use std::sync::Arc;

use crate::diagnostic::{Diagnostic, DiagnosticCode};
use crate::formatter::{DiagnosticFormatter, RenderOptions};
use crate::DiagnosticEngine;

/// Print every diagnostic in `engine` to stderr, picking with-source vs
/// compact rendering per diagnostic. `color = None` → auto-detect via
/// `std::io::stderr().is_terminal()`. Trailing summary line included.
pub fn print_all_smart<C: DiagnosticCode>(engine: &DiagnosticEngine<C>, color: Option<bool>) {
  let color = color.unwrap_or_else(|| std::io::stderr().is_terminal());
  eprint!("{}", format_all_smart(engine, color));
}

/// Format every diagnostic in `engine` to a single string, picking
/// with-source vs compact rendering per diagnostic. Trailing summary
/// line included.
pub fn format_all_smart<C: DiagnosticCode>(engine: &DiagnosticEngine<C>, color: bool) -> String {
  let mut sources: HashMap<Arc<str>, Option<String>> = HashMap::new();
  let mut out = String::new();
  for d in engine.iter() {
    out.push_str(&format_one_smart(d, &mut sources, color));
  }
  out.push_str(&engine.summary(color));
  out
}

fn format_one_smart<C: DiagnosticCode>(
  d: &Diagnostic<C>,
  cache: &mut HashMap<Arc<str>, Option<String>>,
  color: bool,
) -> String {
  let Some(label) = d.primary_label() else {
    return d.format_compact(color);
  };
  if label.span.line == 0 {
    return d.format_compact(color);
  }
  let file = label.span.file.clone();
  let entry = cache.entry(file.clone()).or_insert_with(|| std::fs::read_to_string(&*file).ok());
  match entry {
    Some(src) => {
      let opts = RenderOptions { color, ..Default::default() };
      DiagnosticFormatter::new(d, src).with_options(opts).format()
    },
    None => d.format_compact(color),
  }
}
