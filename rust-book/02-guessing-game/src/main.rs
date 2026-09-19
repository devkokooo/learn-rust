use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, String> {
        if value < 1 || value > 100 {
            return Err(format!("Guess value must be between 1 and 100, got {value}"));
        }
        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let guess: Guess = match Guess::new(guess) {
            Ok(num) => num,
            Err(msg) => {
                println!("{msg}");
                continue;
            },
        };
        let guess = guess.value();

        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
