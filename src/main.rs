use std::{io, fs};
use colored::{Colorize, ColoredString};
use dictionary::Dictionary;
use rand::seq::IteratorRandom;
mod dictionary;

#[tokio::main]
async fn main() {
    let dictionary = Dictionary::new();
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
    let mut game_over = false;
    while guess_count<5 && !game_over {
        let guess = get_guess().await;
        if guess.len() != 5 {
            println!("Please enter a 5 letter word");
        } else if dictionary.invalid_guess(&guess) {
            println!("Please enter a valid word");
        } else {
            let results = check_guess(&guess, &word);
            let output = output_result(results.await, &guess);
            guess_count += 1;
            game_over = word == guess;
            output.await.iter().for_each(|x| print!("{}", x));
            print!("\n");
        }
    }
    if guess_count == 5 && !game_over {
        println!("You lose! the word was {}", word.to_ascii_uppercase());
    } else {
        println!("You win! You guessed the word in {} guesses", guess_count);
    }
    println!("Would you like to play again? (y/n)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");
    response.trim().chars().next().expect("Invalid response").to_ascii_lowercase() == char::from('y')
}

async fn output_result(results: Vec<LetterResult>, guess: &String) -> Vec<ColoredString> {
    let mut output = Vec::new();
    for (i, result) in results.iter().enumerate() {
        match result {
            LetterResult::GREEN => output.push(format!("{}", guess.chars().nth(i).unwrap().to_uppercase()).on_green()),
            LetterResult::YELLOW => output.push(format!("{}", guess.chars().nth(i).unwrap().to_uppercase()).on_yellow()),
            LetterResult::GREY => output.push(format!("{}", guess.chars().nth(i).unwrap().to_uppercase()).on_bright_black())
        }
    }
    output
}

async fn check_guess(guess: &String, word: &String) -> Vec<LetterResult> {
    let mut letter_counts = vec![0; 26];
    word.chars().for_each(|x| letter_counts[x as usize - 97] += 1);

    let mut results = vec![LetterResult::GREY; 5];
    for (i, c) in guess.chars().enumerate() {
        let x = c as usize - 97;
        if c == word.chars().nth(i).unwrap() {
            results[i] = LetterResult::GREEN;
            letter_counts[x] -= 1;
        }
    }

    for (i, c) in guess.chars().enumerate() {
        let x = c as usize - 97;
        if letter_counts[x] > 0 && !matches!(results[i],LetterResult::GREEN) {
            results[i] = LetterResult::YELLOW;
            letter_counts[x] -= 1;
        }
    }
    results
}

async fn get_guess() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    let guess: String = guess.trim().parse()
        .expect("Please type a word!");
    guess.to_ascii_lowercase()
}

async fn get_answer() -> String {
    let fstring = fs::read_to_string("src/resources/wordle_answers.txt").unwrap();
    let lines = fstring.lines();
    let mut rng = rand::thread_rng();
    let random_line = lines.choose(&mut rng).unwrap();
    random_line.trim().to_ascii_lowercase()
}

#[derive(Debug, Clone)]
enum LetterResult {
    GREEN,
    YELLOW,
    GREY
}