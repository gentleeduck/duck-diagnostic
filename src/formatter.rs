use colored::*;
use unicode_width::UnicodeWidthStr;

use crate::diagnostic::{Diagnostic, DiagnosticCode, Label, LabelStyle, Suggestion};
use crate::style::{arrow, bar, code_word, eq_sep, meta_label, paint, paint_label, severity_word};

/// Pre-split source cache. Build once per source string, reuse for many diagnostics.
#[derive(Debug, Clone)]
pub struct SourceCache<'a> {
  lines: Vec<&'a str>,
}

impl<'a> SourceCache<'a> {
  pub fn new(source: &'a str) -> Self {
    Self { lines: source.lines().collect() }
  }

  pub fn line(&self, line_num_1based: usize) -> Option<&str> {
    if line_num_1based == 0 {
      return None;
    }
    self.lines.get(line_num_1based - 1).copied()
  }

  pub fn len(&self) -> usize {
    self.lines.len()
  }

  pub fn is_empty(&self) -> bool {
    self.lines.is_empty()
  }
}

/// Owned variant — allocates a `Vec<String>` internally. Use this when you
/// don't have a `&str` source to borrow (or for back-compat with v0.1).
#[derive(Debug, Clone)]
struct OwnedSource(Vec<String>);

impl OwnedSource {
  fn new(source: &str) -> Self {
    Self(source.lines().map(String::from).collect())
  }
  fn line(&self, n: usize) -> Option<&str> {
    if n == 0 {
      None
    } else {
      self.0.get(n - 1).map(String::as_str)
    }
  }
  fn len(&self) -> usize {
    self.0.len()
  }
}

enum CacheRef<'a, 'src> {
  Borrowed(&'a SourceCache<'src>),
  Owned(OwnedSource),
}

impl<'a, 'src> CacheRef<'a, 'src> {
  fn line(&self, n: usize) -> Option<&str> {
    match self {
      Self::Borrowed(c) => c.line(n),
      Self::Owned(o) => o.line(n),
    }
  }
  fn len(&self) -> usize {
    match self {
      Self::Borrowed(c) => c.len(),
      Self::Owned(o) => o.len(),
    }
  }
}

/// Tunables for rendered output.
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
  /// Tab stop width when expanding tabs in source lines.
  pub tab_width: usize,
  /// Number of context lines printed above + below each label region.
  pub context_lines: usize,
  /// Maximum rendered line width before truncation. `0` disables truncation.
  pub max_line_width: usize,
  /// Use ANSI color codes.
  pub color: bool,
}

impl Default for RenderOptions {
  fn default() -> Self {
    Self { tab_width: 4, context_lines: 0, max_line_width: 0, color: true }
  }
}

pub struct DiagnosticFormatter<'a, 'src, C: DiagnosticCode> {
  diagnostic: &'a Diagnostic<C>,
  cache: CacheRef<'a, 'src>,
  options: RenderOptions,
}

impl<'a, 'src, C: DiagnosticCode> DiagnosticFormatter<'a, 'src, C> {
  /// Construct from a raw source string. For repeated formatting against the
  /// same source, build a [`SourceCache`] once and use [`DiagnosticFormatter::with_cache`].
  pub fn new(diagnostic: &'a Diagnostic<C>, source: &str) -> Self {
    Self {
      diagnostic,
      cache: CacheRef::Owned(OwnedSource::new(source)),
      options: RenderOptions::default(),
    }
  }

  pub fn with_cache(diagnostic: &'a Diagnostic<C>, cache: &'a SourceCache<'src>) -> Self {
    Self { diagnostic, cache: CacheRef::Borrowed(cache), options: RenderOptions::default() }
  }

  pub fn with_options(mut self, options: RenderOptions) -> Self {
    self.options = options;
    self
  }

  fn underline_char(style: LabelStyle) -> char {
    match style {
      LabelStyle::Primary => '^',
      LabelStyle::Secondary => '-',
    }
  }

  /// Pretty (colored) format. Falls back to plain if `options.color = false`.
  pub fn format(&self) -> String {
    if self.options.color {
      self.format_inner(true)
    } else {
      self.format_inner(false)
    }
  }

  /// Plain (no color, deterministic) format. Suitable for CI logs.
  pub fn format_plain(&self) -> String {
    self.format_inner(false)
  }

  fn format_inner(&self, color: bool) -> String {
    let mut out = String::new();
    self.write_header(&mut out, color);
    self.write_labels_grouped(&mut out, color);
    self.write_notes_help(&mut out, color);
    self.write_suggestions(&mut out, color);
    // Trailing blank line so consecutive diagnostics don't visually merge.
    out.push('\n');
    out
  }

  fn write_header(&self, out: &mut String, color: bool) {
    let d = &self.diagnostic;
    out.push_str(&format!(
      "{}: [{}]: {}",
      severity_word(d.severity, color),
      code_word(d.severity, d.code.code(), color),
      d.message,
    ));
    if let Some(u) = d.code.url() {
      out.push_str(&format!(" {}", paint(&format!("(see {u})"), color, |s| s.blue().italic())));
    }
    out.push('\n');
  }

  fn write_labels_grouped(&self, out: &mut String, color: bool) {
    let labels = &self.diagnostic.labels;
    if labels.is_empty() {
      return;
    }

    // Group by file so multi-file diagnostics render as separate sections.
    let mut files: Vec<&str> = Vec::new();
    for l in labels {
      if !files.iter().any(|f| **f == *l.span.file) {
        files.push(&l.span.file);
      }
    }

    for (idx, file) in files.iter().enumerate() {
      let in_file: Vec<&Label> = labels.iter().filter(|l| *l.span.file == **file).collect();
      let primary_in_file = in_file
        .iter()
        .find(|l| l.style == LabelStyle::Primary)
        .copied()
        .or(in_file.first().copied());
      let primary = match primary_in_file {
        Some(l) => l,
        None => continue,
      };

      let loc = if color {
        format!(
          "{}:{}:{}",
          primary.span.file.clone().white().bold(),
          primary.span.line.to_string().white().bold(),
          primary.span.column.to_string().white().bold(),
        )
      } else {
        format!("{}:{}:{}", primary.span.file, primary.span.line, primary.span.column)
      };
      out.push_str(&format!("  {} {}\n", arrow(color), loc));

      self.write_file_section(out, &in_file, color);

      if idx + 1 < files.len() {
        out.push('\n');
      }
    }
  }

  fn write_file_section(&self, out: &mut String, labels: &[&Label], color: bool) {
    // Determine line range to render: min..=max of all labels in this file,
    // padded by context_lines.
    let min_line = labels.iter().map(|l| l.span.line).min().unwrap_or(0);
    let max_line = labels.iter().map(|l| l.span.line).max().unwrap_or(0);
    if min_line == 0 {
      // synthetic span — nothing to render
      return;
    }

    let start = min_line.saturating_sub(self.options.context_lines).max(1);
    let end = (max_line + self.options.context_lines).min(self.cache.len());

    let gutter_w = end.to_string().len().max(2);
    let bar_s = bar(color);
    let blank_gutter = " ".repeat(gutter_w);
    out.push_str(&format!("  {} {}\n", blank_gutter, bar_s));

    for line_num in start..=end {
      let raw = self.cache.line(line_num).unwrap_or("");
      let expanded = expand_tabs(raw, self.options.tab_width);
      let truncated = truncate_line(&expanded, self.options.max_line_width);
      let line_label = format!("{:>w$}", line_num, w = gutter_w);
      let line_label_c = paint(&line_label, color, |s| s.blue().bold());
      out.push_str(&format!("  {} {} {}\n", line_label_c, bar_s, truncated));

      // collect labels touching this line, sorted by start column
      let mut on_line: Vec<&Label> =
        labels.iter().copied().filter(|l| label_touches(l, line_num)).collect();
      if on_line.is_empty() {
        continue;
      }
      on_line.sort_by_key(|l| l.span.column);
      self.write_caret_block(out, &on_line, line_num, raw, gutter_w, color);
    }

    out.push_str(&format!("  {} {}\n", blank_gutter, bar_s));
  }

  /// Render all labels on one source line as a stacked block.
  ///
  /// Layout (rustc-style):
  ///   row 0  : carets for every label  →  message of last (rightmost) label
  ///   row 1  : carets up to label[n-2] →  message of label[n-2]
  ///   …
  ///   row n-1: caret for label[0]      →  message of label[0]
  ///
  /// Each label keeps its own color. Optional per-label `note` renders right
  /// after that label's message row.
  fn write_caret_block(
    &self,
    out: &mut String,
    sorted: &[&Label],
    line_num: usize,
    raw_line: &str,
    gutter_w: usize,
    color: bool,
  ) {
    let infos: Vec<(usize, usize, &Label)> = sorted
      .iter()
      .map(|label| {
        let (col_start, col_end) = label_columns_on_line(label, line_num, raw_line);
        let pad =
          display_width_prefix(raw_line, col_start.saturating_sub(1), self.options.tab_width);
        let len = display_width_range(
          raw_line,
          col_start.saturating_sub(1),
          col_end.saturating_sub(1),
          self.options.tab_width,
        )
        .max(1);
        (pad, len, *label)
      })
      .collect();

    let n = infos.len();
    let bar_s = bar(color);
    let blank_gutter = " ".repeat(gutter_w);

    for k in 0..n {
      let m = n - 1 - k;
      let visible = &infos[..=m];

      // Build the caret row by walking visible labels left→right.
      let mut buf = String::new();
      let mut cursor = 0usize;
      for (pad, len, lbl) in visible {
        while cursor < *pad {
          buf.push(' ');
          cursor += 1;
        }
        let ch = Self::underline_char(lbl.style);
        let underline: String = std::iter::repeat_n(ch, *len).collect();
        buf.push_str(&paint_label(self.diagnostic.severity, lbl.style, &underline, color));
        cursor += *len;
      }

      // Append the message of `m` after the last caret with one space of gap.
      let m_label = visible.last().unwrap().2;
      let line = match &m_label.message {
        Some(msg) => format!(
          "  {} {} {} {}\n",
          blank_gutter,
          bar_s,
          buf,
          paint_label(self.diagnostic.severity, m_label.style, msg, color),
        ),
        None => format!("  {} {} {}\n", blank_gutter, bar_s, buf),
      };
      out.push_str(&line);

      if let Some(note) = &m_label.note {
        let note_c = if color { note.cyan().italic().to_string() } else { format!("note: {note}") };
        out.push_str(&format!(
          "  {} {} {}↳ {}\n",
          blank_gutter,
          bar_s,
          " ".repeat(infos[m].0),
          note_c,
        ));
      }
    }
  }

  fn write_notes_help(&self, out: &mut String, color: bool) {
    let eq = eq_sep(color);
    for note in &self.diagnostic.notes {
      out.push_str(&format!("   {} {}: {}\n", eq, meta_label("note", color), note));
    }
    if let Some(help) = &self.diagnostic.help {
      out.push_str(&format!("   {} {}: {}\n", eq, meta_label("help", color), help));
    }
  }

  fn write_suggestions(&self, out: &mut String, color: bool) {
    if self.diagnostic.suggestions.is_empty() {
      return;
    }
    let eq = eq_sep(color);
    let help = meta_label("help", color);
    for s in &self.diagnostic.suggestions {
      let header = s.message.clone().unwrap_or_else(|| "try this:".to_string());
      out.push_str(&format!("   {} {}: {}\n", eq, help, header));
      self.write_suggestion_diff(out, s, color);
      Self::write_applicability(out, s, color);
    }
  }

  /// Render a suggestion as rustc-style minus/plus diff lines. Falls back to
  /// flat replacement render when the source line isn't available (synthetic
  /// span or out-of-range line).
  fn write_suggestion_diff(&self, out: &mut String, s: &Suggestion, color: bool) {
    let line_num = s.span.line;
    let orig_line = match self.cache.line(line_num) {
      Some(l) => l,
      None => {
        for line in s.replacement.lines() {
          out.push_str(&format!("       {}\n", paint(line, color, |s| s.green())));
        }
        return;
      },
    };

    // Convert 1-based column → byte offset. Use saturating arithmetic to
    // tolerate suggestions slightly off the line end (e.g. column = line.len() + 1).
    let col0 = s.span.column.saturating_sub(1);
    let line_bytes = orig_line.len();
    let start = col0.min(line_bytes);
    let end = (start + s.span.length).min(line_bytes);
    let prefix = &orig_line[..start];
    let suffix = &orig_line[end..];

    // Build rewritten content by splicing replacement between prefix + suffix.
    // First rewritten line includes prefix + first replacement line; subsequent
    // replacement lines stand alone; final replacement line gets suffix appended.
    let repl_lines: Vec<&str> = s.replacement.split('\n').collect();
    let mut new_lines: Vec<String> = Vec::with_capacity(repl_lines.len());
    for (i, r) in repl_lines.iter().enumerate() {
      let head = if i == 0 { prefix } else { "" };
      let tail = if i == repl_lines.len() - 1 { suffix } else { "" };
      new_lines.push(format!("{}{}{}", head, r, tail));
    }

    let last_line = line_num + new_lines.len().saturating_sub(1);
    let gutter_w = last_line.to_string().len().max(2);
    let bar_s = bar(color);
    let blank_gutter = " ".repeat(gutter_w);
    let minus = paint("-", color, |s| s.red().bold());
    let plus = paint("+", color, |s| s.green().bold());

    out.push_str(&format!("  {} {}\n", blank_gutter, bar_s));

    let lbl = format!("{:>w$}", line_num, w = gutter_w);
    out.push_str(&format!(
      "  {} {} {}\n",
      paint(&lbl, color, |s| s.blue().bold()),
      minus,
      paint(orig_line, color, |s| s.red()),
    ));

    for (i, body) in new_lines.iter().enumerate() {
      let lbl = format!("{:>w$}", line_num + i, w = gutter_w);
      out.push_str(&format!(
        "  {} {} {}\n",
        paint(&lbl, color, |s| s.blue().bold()),
        plus,
        paint(body, color, |s| s.green()),
      ));
    }

    out.push_str(&format!("  {} {}\n", blank_gutter, bar_s));
  }

  fn write_applicability(out: &mut String, s: &Suggestion, color: bool) {
    let kind = match s.applicability {
      crate::diagnostic::Applicability::MachineApplicable => "auto-applicable",
      crate::diagnostic::Applicability::MaybeIncorrect => "review needed",
      crate::diagnostic::Applicability::HasPlaceholders => "has placeholders",
      crate::diagnostic::Applicability::Unspecified => return,
    };
    out.push_str(&format!("       ({})\n", paint(kind, color, |s| s.dimmed())));
  }
}

// ---------- helpers ----------

fn label_touches(label: &Label, line: usize) -> bool {
  let start = label.span.line;
  let end_line = end_line_of(label);
  line >= start && line <= end_line
}

/// For a possibly multi-line label, clamp its column range to the given line.
fn label_columns_on_line(label: &Label, line: usize, raw_line: &str) -> (usize, usize) {
  let start_line = label.span.line;
  let line_byte_len = raw_line.len();
  let line_end_col = line_byte_len + 1; // 1-based inclusive end-of-line column

  let start_col = if line == start_line { label.span.column.max(1) } else { 1 };
  let end_col_inclusive = if end_line_of(label) == line {
    if line == start_line {
      // single-line label: column..column+length
      (label.span.column + label.span.length).max(label.span.column + 1)
    } else {
      // last line of multi-line label: end at remaining length on this line
      // we don't have per-line offsets, so just stop at line end
      line_end_col
    }
  } else {
    line_end_col
  };

  (start_col, end_col_inclusive.min(line_end_col).max(start_col + 1))
}

fn end_line_of(label: &Label) -> usize {
  // Without per-line offsets we treat `length` as a byte budget consumed
  // top-down; callers that use multi-line spans should split them into
  // multiple labels for precise rendering. For our purposes the label
  // ends on its start line unless the user attaches multiple labels.
  label.span.line
}

fn expand_tabs(line: &str, tab_width: usize) -> String {
  if !line.contains('\t') {
    return line.to_string();
  }
  let mut out = String::with_capacity(line.len() + tab_width);
  let mut col = 0usize;
  for ch in line.chars() {
    if ch == '\t' {
      let advance = tab_width - (col % tab_width.max(1));
      for _ in 0..advance {
        out.push(' ');
      }
      col += advance;
    } else {
      out.push(ch);
      col += UnicodeWidthStr::width(ch.to_string().as_str());
    }
  }
  out
}

fn truncate_line(line: &str, max: usize) -> String {
  if max == 0 {
    return line.to_string();
  }
  let w = UnicodeWidthStr::width(line);
  if w <= max {
    return line.to_string();
  }
  // keep first max-1 cols, then ellipsis
  let mut out = String::new();
  let mut acc = 0usize;
  for ch in line.chars() {
    let cw = UnicodeWidthStr::width(ch.to_string().as_str());
    if acc + cw + 1 > max {
      break;
    }
    out.push(ch);
    acc += cw;
  }
  out.push('…');
  out
}

/// Display width of `&line[..n_byte_cols]`, accounting for tabs + unicode width.
fn display_width_prefix(line: &str, byte_offset_0based: usize, tab_width: usize) -> usize {
  let mut width = 0usize;
  let mut byte_seen = 0usize;
  for ch in line.chars() {
    if byte_seen >= byte_offset_0based {
      break;
    }
    if ch == '\t' {
      width += tab_width - (width % tab_width.max(1));
    } else {
      width += UnicodeWidthStr::width(ch.to_string().as_str());
    }
    byte_seen += ch.len_utf8();
  }
  width
}

fn display_width_range(
  line: &str,
  start_byte_0based: usize,
  end_byte_0based: usize,
  tab_width: usize,
) -> usize {
  if end_byte_0based <= start_byte_0based {
    return 0;
  }
  let mut width = 0usize;
  let mut byte_seen = 0usize;
  for ch in line.chars() {
    if byte_seen >= end_byte_0based {
      break;
    }
    if byte_seen >= start_byte_0based {
      if ch == '\t' {
        width += tab_width - (width % tab_width.max(1));
      } else {
        width += UnicodeWidthStr::width(ch.to_string().as_str());
      }
    }
    byte_seen += ch.len_utf8();
  }
  width
}
