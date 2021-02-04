use rand::Rng;
use std::process::Command;
use std::io;

struct Sentence {
    word: String,
    found: bool
}

impl Sentence {
    fn create() -> Sentence {
        let word_list: Vec<&str> = vec![
            "hello", "test", "cop", "police", "car", "boat", "boar", "weapon", "deadly", "fire",
        ];
        let word_index = rand::thread_rng().gen_range(0, word_list.len());
        let secret_word = word_list[word_index];
        Sentence {
            word: secret_word.to_string(),
            found: false
        }
    }
    fn check_word(&mut self, word: &str) {
        if self.word == word {
            self.found = true
        }
    }
}

fn main() {
    let mut guess_sentece = Sentence::create();
    let mut display_sentence = String::new();
    let mut guessed_chars: Vec<char> = vec![];
    let mut wins = 0;
    let mut tries = 16;

    loop {
        Command::new("cls");
        if guess_sentece.word.is_empty() || guess_sentece.found {
            guessed_chars.clear();
            wins += 1;
            tries = 16;
            display_sentence = "".to_string();
            guess_sentece = Sentence::create();
        }

        if display_sentence.is_empty() {
            for c in guess_sentece.word.chars() {
                if c != ' ' {
                    display_sentence.push('_');
                }
                else {
                    display_sentence.push(' ');
                }
            }
        }

        println!("Wins: {}\r\nTries: {}", wins, tries);
        println!("Word: {}", display_sentence);

        let mut char_input = String::new();

        println!("Guess a character: ");

        match io::stdin().read_line(&mut char_input) {
            Ok(_) => {
                let input_valid = if char_input == "" || char_input == "\n" || char_input == "\r" || char_input == "\r\n" {false} else {true};
                if input_valid {
                    let chr: char = char_input.chars().nth(0).unwrap();
                    if !guessed_chars.contains(&chr) {
                        tries -= 1;

                        guessed_chars.push(chr);
                        if guess_sentece.word.contains(chr) {
                            println!("'{}' is in the word!", chr);
                            display_sentence = "".to_string();
                            for c in guess_sentece.word.chars() {
                                if guessed_chars.contains(&c) {
                                    display_sentence.push(c);
                                }
                                else {
                                    if c == ' ' {
                                        display_sentence.push(' ');
                                    }
                                    else {
                                        if c == chr {
                                            display_sentence.push(c);
                                        }
                                        else {
                                            display_sentence.push('_');
                                        }
                                    }
                                }
                            }
                        }
                        else {
                            println!("'{}' is not in the word...", chr);
                        }
                        guess_sentece.check_word(&display_sentence);
                    }
                    else {
                        println!("'{}' is already used!", chr);
                    }
                }
                else {
                    println!("Not valid...");
                }
            }
            Err(e) => println!("Something went to shits\r\n \"{}\"", e)
        }
    }
}