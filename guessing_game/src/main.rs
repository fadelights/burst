use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                println!("Not a number. Try again.");
                continue;
            },
            Ok(num) => { num },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
