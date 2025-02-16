mod bulls_and_cows;
mod caesar;
mod secret_number_game;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => secret_number_game::play(),
        2 => match args[1].as_str() {
            "guess_number" => secret_number_game::play(),
            "bulls_and_cows" | "bulls" | "bull" | "cows" | "cow" => bulls_and_cows::play(),
            "caesar" => caesar::encrypt(),
            "decipher" => caesar::decrypt(),
            _ => println!("Game doesn't exist: {}", args[1]),
        },
        _ => println!("Define single game only"),
    }
}
