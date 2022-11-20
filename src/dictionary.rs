
// use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
pub struct Dictionary {
    words: Box<HashSet<String>>
}
impl Dictionary {
    pub async fn new() -> Dictionary {
        let mut words = HashSet::new();
        let file = fs::read_to_string("src/resources/dictionary.txt").expect("Unable to read dictionary.txt");
        // file.lines().par_bridge().for_each(|line| { words.clone().insert(line.to_string()); });
        file.lines().for_each(|line| { words.insert(line.to_string()); });
        Dictionary {
            words: Box::new(words)
        }
    }
    pub fn invalid_guess(&self, guess: &String) -> bool {
        if self.words.len() == 0 {
            panic!("empty dictionary");
        }
        guess.chars().any(|x| !x.is_ascii_alphabetic()) || !self.words.contains(guess)
    }
}

// pub async fn get_dict_set() -> Box<HashSet<String>> {
//     let dictionary = HashSet::new();
//     let fstring = fs::read_to_string("src/resources/dictionary.txt").unwrap();
//     let lines = fstring.lines();
//     lines.par_bridge().for_each(|line| { dictionary.clone().insert(line.to_string()); });
//     Box::new(dictionary)
// }

// pub fn invalid_guess(guess: &String) -> bool {
//     guess.chars().any(|x| !x.is_ascii_alphabetic()) || std::fs::read_to_string("src/resources/dictionary.txt").unwrap().lines().all(|line| *guess != line.to_string())
// }