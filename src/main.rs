use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

fn main() {
    // Get the list of answers and list of allowed guess words into vectors so we can index them later
    let answers = get_words("words/answers.txt");
    let allowed = get_words("words/allowed.txt");

    println!("answers[0] = {}", answers[0]);
    println!("allowed[0] = {}", allowed[0]);

    let answer = get_answer(&answers);
    println!("\nThe answer for this session's rustle is {}\n", answer);
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
