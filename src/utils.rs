pub fn pluralize(word: &str, count: usize) -> &str {
  if count == 1 {
    return word;
  }
  match word {
    "error" => "errors",
    "warning" => "warnings",
    _ => word,
  }
}
