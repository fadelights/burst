/* The `io` module is part of the standard library (`std`).
You import it using the notation `std::io` and access it's
functions using the same notation; e.g. `io::stdin()`.

If we don't want to import a module, we can still use its
functions by using the full specifier: `std::io::stdin()`

To import multiple modules from the same library, you can use
the {} notation as shown below. */
use std::{cmp::Ordering, io};

/* The `rand` library is a 3rd-party library.
You can view it in the project's dependency list in Cargo.toml.

While the `Rng` trait isn't explicitly used in the code, it
defines methods that are implemented by random number generators
and as such should be imported. */
use rand::Rng;

fn main() {
    println!("Guess the Number!"); // Semicolons are a thing in Rust!

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        // You can have multi-line statements for increased readability
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Err(_) => {
                println!("Not a number. Try again.");
                continue;
            },
            Ok(num) => num, // Commas can be added at the end of lists too
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
