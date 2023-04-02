use std::{collections::HashSet, io};
mod art;
mod words;

pub struct Game {
    picked_word: String,
    wrong_letters: HashSet<char>,
    art_steps: Vec<&'static str>,
    correct_letters: HashSet<char>,
}

enum GameError {
    TooManyLetters,
    LetterUsed,
    FailedToReadInput,
    GameOver,
}

impl Game {
    pub fn build() -> Self {
        let art_steps = art::get_art();
        let picked_word = words::get_random_word();
        Game {
            art_steps,
            wrong_letters: HashSet::with_capacity(picked_word.len()),
            correct_letters: HashSet::with_capacity(picked_word.len()),
            picked_word,
        }
    }

    fn check_letter(&mut self, letter: char) {
        if !self.picked_word.contains(letter) {
            self.wrong_letters.insert(letter);
        } else {
            self.correct_letters.insert(letter);
        }
    }

    fn read_input() -> Result<char, GameError> {
        println!("Enter a single letter: ");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .or(Err(GameError::FailedToReadInput))?;

        if line.trim().len() != 1 {
            return Err(GameError::TooManyLetters);
        }
        let letter = line.chars().next().ok_or(GameError::FailedToReadInput)?;
        Ok(letter)
    }

    fn next(&mut self, letter: char) -> Result<bool, GameError> {
        if self.correct_letters.contains(&letter) || self.wrong_letters.contains(&letter) {
            println!("You already guessed this letter");
            return Err(GameError::LetterUsed);
        }

        self.check_letter(letter);

        if self.wrong_letters.len() == self.art_steps.len() - 1 {
            return Err(GameError::GameOver);
        } else {
            let guessed_letters = self.get_guessed_letters();
            if !guessed_letters.contains('_') {
                println!("You guessed the word: {}", self.picked_word);
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn run(mut self) {
        if std::env::var("DEBUG").is_ok() {
            println!("Picked word is {}", self.picked_word);
        }
        loop {
            self.print_art();
            println!("{}", self.get_letters_state());
            println!("{}", self.get_guessed_letters());

            let input = if let Ok(input) = Self::read_input() {
                input
            } else {
                continue;
            };

            match self.next(input) {
                Err(GameError::GameOver) => {
                    self.print_art();
                    println!(
                        "{}You lost{}. Your word was {}",
                        art::RED,
                        art::RESET,
                        self.picked_word
                    );
                    break;
                }
                Ok(terminate) => {
                    if terminate {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    fn get_guessed_letters(&self) -> String {
        let output: String = self
            .picked_word
            .chars()
            .map(|c| {
                if self.correct_letters.contains(&c) {
                    String::from(c)
                } else {
                    String::from("_")
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
        output
    }

    fn print_art(&self) {
        if let Some(art) = self.art_steps.get(self.wrong_letters.len()) {
            println!("{art}");
        }
    }

    fn get_letters_state(&self) -> String {
        let letters = words::get_ascii_leters()
            .iter()
            .map(|c| {
                let prefix = if self.correct_letters.contains(c) {
                    art::GREEN
                } else if self.wrong_letters.contains(c) {
                    art::RED
                } else {
                    ""
                };
                return format!("{prefix}{c}{}", art::RESET);
            })
            .collect();

        letters
    }
}
