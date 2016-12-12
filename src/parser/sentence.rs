pub fn parse(sentence: String) -> (String, Vec<String>) {
  let mut words = sentence.split_whitespace();

  let command: String = words
    .nth(0)
    .map(|s| s.to_lowercase())
    .unwrap_or(String::from("unknown"));

  let params: Vec<String> = words
    .skip(0)
    .map(|s| s.to_lowercase())
    .collect();

  (command, params)
}
