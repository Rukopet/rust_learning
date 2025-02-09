use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn play() {
    let secret_number = rand::rng().random_range(0..=100);

    println!("Welcome to the guessing game!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Something worng with input");

        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Enter a number! {}", e);
                continue;
            }
        };

        match guess_num.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
            Ordering::Greater => println!("Secret number is lower, enter new!"),
            Ordering::Less => println!("Secret number is greater; enter new!"),
        }
    }
}
