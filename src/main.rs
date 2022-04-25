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

    // let answer = get_answer(&answers);
    let answer = "abele".to_string();

    // Enter game
    let mut num_guesses: i8 = 0;
    let mut game_win = false;
    while num_guesses < 6 {
        print_game_state(&guesses, &answer);

        println!("Guess a word:");

        // Get guess from standard input
        let guess: String = read!();

        // Make the user guess again if the word isn't five letters long or isn't in either the answers or allowed lists
        if guess.len() != 5 || !is_word_valid(&guess, &answers, &allowed) {
            continue;
        } else {
            // Guess is exactly 5 letters and is valid

            // Add guess to vector of previous guesses
            guesses.push((&guess).to_string());
            num_guesses += 1;

            // If the user guesses correctly, they win and the game is over
            if guess == answer {
                game_win = true;
                break;
            }
        }
    }

    // Print the result of the game once the loop exits
    game_over(&guesses, &answer, &num_guesses, game_win);
}

/// Prints the summary of the game once complete, including the number of guesses (out of 6) and the block emojis
/// to represent each guess
///
/// # Arguments
/// * `guesses` - a reference to a vector of previous guesses in string form
/// * `answer` - a reference to the answer word in string form
/// * `num_guesses` - a reference to the number of guesses the player made until they either won or lost the game
/// * `game_win` - a boolean representing whether the game was won or lost
fn game_over(guesses: &Vec<String>, answer: &String, num_guesses: &i8, game_win: bool) {
    // Print all the guesses the user has made
    print_game_state(guesses, answer);
    // Print either the number of guesses made (if win) or an X (if lose)
    if game_win {
        println!("Rustle {}/6", num_guesses);
    } else {
        println!("Rustle X/6");
    }
    // Print the answer
    println!("Word: {}", answer);
    // Loop through all guesses and print them as emoji blocks
    for guess in guesses.iter() {
        print_guess(guess, answer, true);
    }
    println!("");  // Add a newline
}

/// Prints the current state of the game, including the title and the result of all previous guesses
///
/// # Arguments
/// * `guesses` - a reference to a vector of previous guesses in string form
/// * `answer` - a reference to the answer word in string form
fn print_game_state(guesses: &Vec<String>, answer: &String) {
    // Clear the console, place the cursor at the first row and column of the terminal
    print!("\x1B[2J\x1B[1;1H");

    println!("Rustle\nBy Dylan Tuttle\n");

    // Loop through each guess made and print it with highlighting
    for guess in guesses.iter() {
        print_guess(guess, answer, false);
    }
    println!("");  // Add a newline
}

/// Prints the result of a particular guess, highlighting each letter either green, yellow, or not at all
///
/// # Arguments
/// * `guess` - a reference to an array of bytes corresponding to the guess word
/// * `answer` - a reference to an array of bytes corresponding to the answer word
/// * `blocks` - a boolean representing whether the guess word should be printed or the block emojis for game over
fn print_guess(guess: &String, answer: &String, blocks: bool) {
    let guess_bytes = guess.as_bytes();
    let answer_bytes = answer.as_bytes();

    // Loop through each letter in the guess
    for i in 0..5 {
        // Turn the letter back into a string so it can be printed
        let guess_i = vec![guess_bytes[i]];
        let guess_i = String::from_utf8(guess_i).unwrap();

        if is_green(&i, &guess_bytes, &answer_bytes) {
            // Print the letter green
            if blocks {
                print!("🟩");
            } else {
                print!("{}", guess_i.bright_green());
            }
            io::stdout().flush().unwrap();  // Ensures the console prints without a newline

        } else if is_yellow(&i, &guess_bytes, &answer_bytes) {
            // Print the letter yellow
            if blocks {
                print!("🟨");
            } else {
                print!("{}", guess_i.yellow());
            }
            io::stdout().flush().unwrap();  // Ensures the console prints without a newline
            
        } else {
            // Print the letter with no color
            if blocks {
                print!("⬛");
            } else {
                print!("{}", guess_i);
            }
            io::stdout().flush().unwrap();  // Ensures the console prints without a newline
        }
    }
    println!("");  // Print a newline at the end of the guess
}

/// Returns a boolean representing if a particular letter in a guessed word should be printed green
/// (indicating that the letter can be found in the answer at this position)
///
/// # Arguments
/// * `index` - a reference to the index of the letter with respect to the guess word
/// * `guess` - a reference to an array of bytes corresponding to the guess word
/// * `answer` - a reference to an array of bytes corresponding to the answer word
fn is_green(index: &usize, guess: &[u8], answer: &[u8]) -> bool {
    guess[*index] == answer[*index]
}

/// Returns a boolean representing if a particular letter in a guessed word should be printed yellow
/// (indicating that the letter can be found in the answer, but not at this position)
///
/// # Arguments
/// * `index` - a reference to the index of the letter with respect to the guess word
/// * `guess` - a reference to an array of bytes corresponding to the guess word
/// * `answer` - a reference to an array of bytes corresponding to the answer word
fn is_yellow(index: &usize, guess: &[u8], answer: &[u8]) -> bool {
    if answer.contains(&guess[*index]) {
        // The letter is found somewhere in the answer word, but not at this index
        // We need to prevent incorrect yellow letters
        // For example, if the answer is point and the guess is foods, the first o should be green and the second o should be grey
        // For example, if the answer is spite and the guess is seedy, the first e should be yellow and the second e should be grey
        
        // Begin by checking the number of letters which match our letter, if there are any
        let mut num_dups = 0;
        let mut num_matches_answer = 0;
        for i in 0 .. 5 {
            // Count the number of letters before (and equal to) this one which match our letter
            if guess[i] == guess[*index] && i <= *index {
                num_dups += 1;
            }
            // count the number of letters in the answer which match our letter
            if answer[i] == guess[*index] {
                num_matches_answer += 1;
            }
        }
        
        if num_matches_answer >= num_dups {
            // For example, if there are two 'o's in the guess, and two or more 'o's in the answer,
            // then we don't have to worry about incorrect yellows, unless we have a green letter ahead of us

            // Check if there is a green letter ahead of us
            for i in *index .. 5 {
                if answer[i] == guess[*index] && is_green(&i, guess, answer) {
                    num_dups += 1;

                    if  num_dups > num_matches_answer {
                        // For example, if our guess has two 'o's, and there is only one 'o' in the answer, but it aligns
                        // with the second 'o' in the guess (such that it is green), then at this point in the function
                        // we are looking at the first 'o' and realizing we shouldn't color it yellow because the second
                        // 'o' already figured out where it is
                        return false;
                    }
                }
            }
            return true;
        } else {
            // There are more of our letter in the guess than in the answer
            return false;
        }
    } else {
        false
    }
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
