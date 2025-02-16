use std::io;

use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
pub enum GameError {
    DuplicateDigits,
    InvalidLength,
}

#[derive(Debug)]
struct GameResult {
    bulls: u8,
    cows: u8,
}

pub fn play() {
    println!("Welcome to Bulls and Cows!");
    println!("Enter a 4-digit number with unique digits (0-9):");
    let secret_number = generate_secret_number();
    let mut buff = String::new();

    io::stdin()
        .read_line(&mut buff)
        .expect("something went wrong");

    loop {
        let input = buff.trim();

        // Улучшенная проверка ввода
        if let Some(invalid_char) = input.chars().find(|c| !c.is_ascii_digit()) {
            println!(
                "Invalid character '{}' found. Please use only digits 0-9.",
                invalid_char
            );
            buff.clear();
            io::stdin()
                .read_line(&mut buff)
                .expect("something went wrong");
            continue;
        }

        // Преобразуем строку в вектор чисел
        let current_number: Vec<u8> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        match bc_cmp(&secret_number, &current_number) {
            Ok(result) if result.bulls == 4 => {
                println!("Congratulations! You've won!");
                break;
            }
            Ok(result) => {
                println!("Bulls: {}, Cows: {}", result.bulls, result.cows);
            }
            Err(e) => println!("Error: {:?}", e),
        }

        buff.clear();
        io::stdin()
            .read_line(&mut buff)
            .expect("something went wrong");
    }
}

fn bc_cmp(secret_number: &Vec<u8>, current_number: &Vec<u8>) -> Result<GameResult, GameError> {
    let mut res = GameResult { bulls: 0, cows: 0 };
    let mut seen = vec![false; 10];

    if current_number.len() != 4 {
        return Err(GameError::InvalidLength);
    }

    for (i, &curr_digit) in current_number.iter().enumerate() {
        // Проверка на дубликаты
        if seen[curr_digit as usize] {
            return Err(GameError::DuplicateDigits);
        }
        seen[curr_digit as usize] = true;

        // Подсчет быков и коров
        if curr_digit == secret_number[i] {
            res.bulls += 1;
        } else if secret_number.contains(&curr_digit) {
            res.cows += 1;
        }
    }

    Ok(res)
}

fn generate_secret_number() -> Vec<u8> {
    let mut numbers: Vec<u8> = (0..=9).collect();
    let mut rng_func = rng();
    numbers.shuffle(&mut rng_func);
    numbers[0..4].to_vec()
}
