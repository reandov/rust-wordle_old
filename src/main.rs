use std::io;
use std::process;

use rust_wordle::compare_words;
use rust_wordle::CurrentGame;

fn main() {
    let game = CurrentGame::new().unwrap_or_else(|err| {
        eprintln!("Failed at: {}", err);
        process::exit(1);
    });

    println!("The current game is: {:?}", game);

    loop {
        println!("Input your word");

        let mut current_guess = String::new();

        io::stdin()
            .read_line(&mut current_guess)
            .expect("Failed to read current_guess.");

        let current_guess: String = match current_guess.trim().to_lowercase().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };

        println!(
            "Your current_guess: {}. Current word: {}. It's equal? {}",
            current_guess,
            game.current_word,
            current_guess.eq(&game.current_word)
        );

        println!("{:?}", compare_words(&game.current_word, &current_guess));

        break;
    }
}
