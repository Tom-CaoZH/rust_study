// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use rand::seq::index::IndexVecIntoIter;
use std::fs;
use std::io;
use std::io::Write;
use std::sync::mpsc::RecvError;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";
const RECORD_GARBAGE: char = '-';

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    let mut recorded = String::new();
    let mut guessed = String::new();
    let length = secret_word.len();
    let mut i = 0;
    let mut guess_num = NUM_INCORRECT_GUESSES;
    let mut flag; // to indicate whether we find a target number
    while i < length {
        recorded.push(RECORD_GARBAGE);
        i += 1;
    }
    println!("Welcome to TC Hangman!");

    while guess_num > 0 {
        i = 0;
        flag = true;

        println!("The word so far is {}",recorded);
        println!("You have guessed the following letters : {}",guessed);
        println!("You have {} guess left",guess_num);
        println!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        let target = guess.chars().nth(0).unwrap();
        while i < length {
            if secret_word_chars[i] == target {
                recorded.replace_range(i..(i+1), &guess[..1]);
                flag = false;
                break;
            }
            i += 1;
        }

        if flag {
            guess_num -= 1;
            println!("Sorry, that letter is not in the word");
        }

        guessed = guessed + &guess[..1];
    }
}
