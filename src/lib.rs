use std::io;
mod art;
mod words;

pub struct Game {
    picked_word: String,
    wrong_guesses: usize,
    art_steps: Vec<&'static str>,
    guessed_letters: Vec<char>,
    finished: bool,
}

impl Game {
    pub fn new() -> Game {
        let art_steps = art::get_art();
        let picked_word = words::get_random_word();
        Game {
            art_steps,
            wrong_guesses: 0,
            guessed_letters: Vec::with_capacity(picked_word.len()),
            picked_word,
            finished: false,
        }
    }

    fn check_letter(&mut self, letter: char) {
        if !self.picked_word.contains(letter) {
            self.wrong_guesses += 1;
        } else {
            self.guessed_letters.push(letter)
        }
    }

    pub fn next(&mut self) {
        println!("Enter a single letter: ");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        if line.trim().len() != 1 {
            panic!("expected exactly one character");
        }
        let letter = line.chars().next().expect("expected exactly one character");
        if self.guessed_letters.contains(&letter) {
            println!("You already guessed this letter");
            return;
        }

        self.check_letter(letter);

        if self.wrong_guesses == self.art_steps.len() - 1 {
            self.print_art();
            println!("You lost");
            self.finished = true;
            return;
        } else {
            self.print_art();
            let output = self.guess_output();
            println!("{output}");

            if !output.contains('_') {
                println!("You guessed the word: {}", self.picked_word);
                self.finished = true;
                return;
            }
        }
    }

    pub fn run(mut self) {
        // println!("Picked word is {}", self.picked_word);
        while !self.finished {
            self.next();
        }
    }

    fn guess_output(&self) -> String {
        let output: String = self
            .picked_word
            .chars()
            .map(|c| {
                if self.guessed_letters.contains(&c) {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        output
    }

    fn print_art(&self) {
        let art = self.art_steps[self.wrong_guesses];
        println!("{art}");
    }
}
