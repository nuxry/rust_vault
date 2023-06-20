// Guessing The Number: Normal Edition
// 11.06.2023

use rand::Rng;
use std::io;

fn modulus(n: i32) -> i32 {
    if n < 0 {
        return n * (-1);
    } else {
        return n;
    }
}

pub fn game() -> ! {
    println!("Welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 and 10. Can you guess it?");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=5);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        let diff: i32 = modulus(guess - secret_number);

        if diff > 1 && diff < 5 {
            println!("Small.")
        } else if diff > 10 && diff < 15 {
            println!("");
        }
    }
}

// TO BE DONE LATER!!!
// determine if small, too small, big, too big
