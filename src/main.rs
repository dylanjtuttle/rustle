use std::fs::File;
use std::io::{self, BufRead};
use std::io::Write;
use std::path::Path;
use rand::Rng;
use text_io::read;
use colored::*;

fn main() {
    // Get the list of answers and list of allowed guess words into vectors so we can index them later
    let answers = get_words("words/answers.txt");
    let allowed = get_words("words/allowed.txt");

    let mut guesses: Vec<String> = Vec::new();  // Vector containing the user's guesses

    let answer = get_answer(&answers);

    // Enter game
    let mut num_guesses: i8 = 0;
    let mut game_win = false;
    while num_guesses < 6 {
        print_game_state(&guesses, &answer);

        println!("Guess a word:");

        let guess: String = read!();

        if guess.len() != 5 || !is_word_valid(&guess, &answers, &allowed) {
            continue;
        } else {
            // Guess is exactly 5 letters and is valid
            guesses.push((&guess).to_string());
            num_guesses += 1;
            if guess == answer {
                game_win = true;
                break;
            }
        }
    }

    game_over(&guesses, &answer, &num_guesses, game_win);
}

fn game_over(guesses: &Vec<String>, answer: &String, num_guesses: &i8, game_win: bool) {
    print_game_state(guesses, answer);
    if game_win {
        println!("Rustle {}/6", num_guesses);
    } else {
        println!("Rustle X/6");
    }
    println!("Word: {}", answer);
    for guess in guesses.iter() {
        print_guess(guess, answer, true);
    }
    println!("");
}

fn print_game_state(guesses: &Vec<String>, answer: &String) {
    // Clear the console, place the cursor at the first row and column of the terminal
    print!("\x1B[2J\x1B[1;1H");

    println!("Rustle\nBy Dylan Tuttle\n");
    for guess in guesses.iter() {
        print_guess(guess, answer, false);
    }
    println!("");
}

fn print_guess(guess: &String, answer: &String, blocks: bool) {
    let guess_bytes = guess.as_bytes();
    let answer_bytes = answer.as_bytes();

    for i in 0..5 {
        let guess_i = vec![guess_bytes[i]];
        let guess_i = String::from_utf8(guess_i).unwrap();

        if guess_bytes[i] == answer_bytes[i] {
            // Print the letter green
            if blocks {
                print!("🟩");
            } else {
                print!("{}", guess_i.bright_green());
            }
            io::stdout().flush().unwrap();
        } else if answer_bytes.contains(&guess_bytes[i]) {
            // Print the letter yellow
            if blocks {
                print!("🟨");
            } else {
                print!("{}", guess_i.yellow());
            }
            io::stdout().flush().unwrap();
        } else {
            // Print the letter with no color
            if blocks {
                print!("⬛");
            } else {
                print!("{}", guess_i);
            }
            io::stdout().flush().unwrap();
        }
    }
    println!("");
}

/// Returns a boolean representing whether a word is in either the list of allowed words or the list of answer words
///
/// # Arguments
/// * `word` - a reference to a string containing the word we want to check the validity of
/// * `answers` - a reference to the vector of answer words
/// * `answers` - a reference to the vector of allowed guessing words
fn is_word_valid(word: &String, answers: &Vec<String>, allowed: &Vec<String>) -> bool {
    answers.contains(word) || allowed.contains(word)
}

/// Returns a randomly selected word from the provided vector
///
/// # Arguments
/// * `answers` - a reference to a vector of strings containing all possible answers
fn get_answer(answers: &Vec<String>) -> String {
    // Generate a random number between 0 and the number of answers in the vector minus 1
    let index = rand::thread_rng().gen_range(0..answers.len() - 1);

    // Return word at index
    let answer = &answers[index];
    answer.to_string()
}

/// Returns a vector of strings given a filename to read lines from
///
/// # Arguments
/// * `filename` - a constant string literal representing the path to a file containing words to read from
fn get_words(filename: &str) -> Vec<String> {

    // Initialize vector to hold words
    let mut words: Vec<String> = Vec::new();  // Vector containing words

    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // Append the string contained on the current line to the vector of words
                words.push(ip);
            }
        }
    }

    // Return vector of words
    words
}

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
