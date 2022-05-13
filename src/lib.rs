use rand::Rng;

const WORDS: [&str; 20] = [
    "candy", "water", "crane", "royal", "piano", "jazzy", "pizza", "lunch", "sharp", "serve",
    "stone", "earth", "actor", "above", "bench", "delay", "faith", "paint", "order", "woman",
];

#[derive(Debug)]
pub struct CurrentGame {
    pub current_word: String,
    pub current_guess: String,
    pub trials: u16,
}

impl CurrentGame {
    pub fn new() -> Result<CurrentGame, &'static str> {
        let current_word = select_word();
        let current_guess = "";
        let trials = 5;

        Ok(CurrentGame {
            current_word,
            current_guess: current_guess.to_string(),
            trials,
        })
    }
}

pub fn select_word() -> String {
    let rand_index = rand::thread_rng().gen_range(0..WORDS.len());

    WORDS[rand_index].to_string()
}

pub fn compare_words(current_word: &str, current_guess: &str) -> (bool, Vec<char>, Vec<char>) {
    let mut is_equal = true;
    let mut valid_chars: Vec<char> = Vec::new();
    let mut invalid_chars: Vec<char> = Vec::new();

    for i in 0..5 {
        if !(current_guess.chars().nth(i) == current_word.chars().nth(i)) {
            is_equal = false;
        }
        if current_word.contains(current_guess.chars().nth(i).unwrap()) {
            valid_chars.push(current_guess.chars().nth(i).unwrap())
        } else {
            invalid_chars.push(current_guess.chars().nth(i).unwrap())
        }
    }

    (is_equal, valid_chars, invalid_chars)
}

#[cfg(test)]
mod tests {
    use crate::{compare_words, WORDS};

    #[test]
    fn should_choose_random_word() {
        let rand_index = 11;

        assert_eq!("earth", WORDS[rand_index])
    }

    #[test]
    fn current_guess_to_be_equal() {
        let current_word = "crane";
        let current_guess = "crane";

        assert_eq!(true, compare_words(&current_word, &current_guess).0);
    }
}
