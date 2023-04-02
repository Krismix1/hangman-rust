pub fn get_art() -> Vec<&'static str> {
    vec![
        r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
    ]
}

// https://stackoverflow.com/a/33139393
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";

pub fn print_red(s: &str) {
    println!("{}{}{}", RED, s, RESET);
}

pub fn format_red(s: &str) -> String {
    format!("{}{}{}", RED, s, RESET)
}

pub fn format_green(s: &str) -> String {
    format!("{}{}{}", GREEN, s, RESET)
}
