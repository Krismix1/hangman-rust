// https://stackoverflow.com/a/33139393
pub const RESET: &str = "\x1b[0m";
pub const GREEN: &str = "\x1b[32m";
pub const RED: &str = "\x1b[31m";

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
