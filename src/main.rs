use rust_wordle;

fn main() {
    let selected_word = rust_wordle::select_word();

    println!("The selected word was: {}", selected_word);
}
