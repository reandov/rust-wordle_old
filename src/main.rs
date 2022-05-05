use std::process;

use rust_wordle::CurrentGame;

fn main() {
    let game = CurrentGame::new().unwrap_or_else(|err| {
        eprintln!("Failed at: {}", err);
        process::exit(1);
    });

    println!("The current game is: {:?}", game);
}
