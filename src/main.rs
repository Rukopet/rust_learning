mod secret_number_game;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => secret_number_game::play(),
        2 => match args[1].as_str() {
            "guess_number" => secret_number_game::play(),
            _ => println!("Game doesn't exists {}", args[1]),
        },
        _ => println!("Define single game only"),
    }
}
