use std::{io, fs};
use colored::{Colorize, ColoredString};
use dictionary::Dictionary;
use rand::seq::IteratorRandom;
mod dictionary;

#[tokio::main]
async fn main() {
    let dictionary = Dictionary::init();
    println!("Welcome to wordle clone by Nate Kimball!");
    println!("Enter 5 letter guesses below to play!");

    let mut play_again = true;
    let dictionary = dictionary.await;
    while play_again {
        let answer = get_answer();
        println!("WORDLE");
        play_again = launch_game(answer.await, &dictionary).await;
    }
}

async fn launch_game(word: String, dictionary: &Dictionary) -> bool {
    let mut guess_count = 0;
    let mut won = false;
    while guess_count<6 && !won {
        let guess = get_guess().await;
        if guess.len() != 5 {
            println!("Please enter a 5 letter word");
        } else if dictionary.invalid_guess(&guess) {
            println!("Please enter a valid word");
        } else {
            let results = check_guess(&guess, &word);
            let output = output_result(results.await, &guess);
            guess_count += 1;
            won = word == guess;
            output.await.iter().for_each(|x| print!("{}", x));
            print!("\n");
        }
    }
    if won {
        println!("You win! You guessed the word in {} guesses", guess_count);
    } else {
        println!("You lose! the word was {}", word.to_ascii_uppercase());
    }
    println!("Would you like to play again? (y/n)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Fa`iled to read line");
    response.trim().chars().next().expect("Invalid response").to_ascii_lowercase() == char::from('y')
}

async fn output_result(results: [LetterResult;5], guess: &String) -> Vec<ColoredString> {
    let mut output = Vec::with_capacity(5);
    for (i, result) in results.iter().enumerate() {
        match result {
            LetterResult::GREEN => output.push(format!("{}", guess.chars().nth(i).unwrap().to_uppercase()).on_green()),
            LetterResult::YELLOW => output.push(format!("{}", guess.chars().nth(i).unwrap().to_uppercase()).on_yellow()),
            LetterResult::GREY => output.push(format!("{}", guess.chars().nth(i).unwrap().to_uppercase()).on_bright_black())
        }
    }
    output
}

async fn check_guess(guess: &String, word: &String) -> [LetterResult; 5] {
    let mut letter_counts = [0; 26];
    let mut results = [LetterResult::GREY; 5];
    word.chars().enumerate().for_each(|(i,c)| {
        if c != guess.chars().nth(i).unwrap() {
            letter_counts[c as usize - 97] += 1;
        } else {
            results[i] = LetterResult::GREEN;
        }
    });
    guess.chars().enumerate().for_each(|(i,c)| {
        let x = c as usize - 97;
        if letter_counts[x] > 0 && !matches!(results[i],LetterResult::GREEN) {
            results[i] = LetterResult::YELLOW;
            letter_counts[x] -= 1;
        }
    });
    results
}

async fn get_guess() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess.trim().to_ascii_lowercase()
}

async fn get_answer() -> String {
    let fstring = fs::read_to_string("src/resources/wordle_answers.txt").unwrap();
    let lines = fstring.lines();
    let random_line = lines.choose(&mut rand::thread_rng()).unwrap();
    random_line.trim().to_ascii_lowercase()
}

#[derive(Copy, Debug, Clone)]
enum LetterResult {
    GREEN,
    YELLOW,
    GREY
}