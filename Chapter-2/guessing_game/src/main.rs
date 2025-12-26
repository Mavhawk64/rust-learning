use std::io::{self, Write};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    loop {
        let ans: u8 = rand::rng().random_range(0..=255) as u8;

        // println!("The secret number is {}", ans);

        loop {
            let guess: u8 = input("Pick a number 0-255: ")
                .trim()
                .parse::<u8>()
                .expect("Please type a number!");

            println!("You guessed: {guess}");

            if guess == ans {
                println!("You win!");
                break;
            }
            if guess < ans {
                println!("Too low!");
            } else {
                println!("Too high!");
            }
        }

        if input("Would you like to try again? (y/n): ").chars().nth(0) != Some('y') {
            break;
        }
    }
}

fn input(txt: &str) -> String {
    print!("{}", txt); // This doesn't normally print out until a new line is prepared.

    // Manually flush stdout to ensure the prompt appears before read_line
    io::stdout().flush().expect("Failed to flush stdout");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess;
}
