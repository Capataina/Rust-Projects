use rand::Rng;
use std::io;
use std::io::prelude::*;

fn main() {
    let mut guessed_number: String = String::new();
    let mut current_difficulty: i8 = 2;
    println!(
        "Welcome to Random Number Game! Your goal is to predict the number from the given range!"
    );

    loop {
        guessed_number.clear();

        let rng: i32 = rand::rng().random_range(0..current_difficulty as i32);

        print!("Guess the number: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("TODO: panic message");

        println!("{}", guessed_number);

        if guessed_number.trim() == "LET ME OUTTT" {
            break;
        }

        if let Ok(num) = guessed_number.trim().parse::<i32>() {
            // num is a valid number
            if num == rng {
                println!("Well done, keep going!");
                current_difficulty += 1;
            } else {
                println!("Wrong! The number was {}.", rng);
                current_difficulty = 2;
            }
        } else {
            println!("⚠️ Please enter a valid number!");
        }
    }
}
