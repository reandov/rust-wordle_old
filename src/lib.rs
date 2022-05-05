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

#[cfg(test)]
mod tests {
    use crate::WORDS;

    #[test]
    fn should_choose_random_word() {
        let rand_index = 11;

        assert_eq!("earth", WORDS[rand_index])
    }
}
