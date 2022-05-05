use std::cmp::Eq;
use std::io;
use std::process;

use rust_wordle::CurrentGame;

fn main() {
    let game = CurrentGame::new().unwrap_or_else(|err| {
        eprintln!("Failed at: {}", err);
        process::exit(1);
    });

    println!("The current game is: {:?}", game);

    loop {
        println!("Input your word");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess.");

        let guess: String = match guess.trim().to_lowercase().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };

        println!(
            "Your guess: {}. Current word: {}. It's equal? {}",
            guess,
            game.current_word,
            guess.eq(&game.current_word)
        );

        break;
    }
}
