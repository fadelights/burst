use std::io::Write;
use std::{cmp, f64, process};

fn main() {
    println!("Hello, Playa!");

    print!("Please input the range of the generated random number, e.g. 1,100: ");
    std::io::stdout().flush().expect("Error printing to stdout");

    let lo: u32;
    let hi: u32;
    (lo, hi) = get_range();
    let secret = gen_secret(lo, hi);

    // TODO: Research proper convesion between fp and int types
    let maxguesses: f64 = f64::log2((hi - lo + 1).into());
    let maxguesses: u32 = f64::floor(maxguesses) as u32; // it's safe because hi and lo were u32 to begin with
    let mut counter: u32 = 0;

    while counter <= maxguesses {
        println!("Number of guesses left: {}", maxguesses - counter + 1);
        counter += 1;

        print!("Please input your guess: ");
        std::io::stdout().flush().expect("Error printing to stdout");
        let guess = get_guess();

        if !(lo <= guess && guess <= hi) {
            panic!("Your guess is not in range!");
        }

        match guess.cmp(&secret) {
            cmp::Ordering::Less => println!("Too small nigga!"),
            cmp::Ordering::Greater => println!("Too big nigga!"),
            cmp::Ordering::Equal => {
                println!("Juuust right homie.");
                println!("You got it right on guess #{counter}");
                break;
            }
        }
    }
}

fn get_range() -> (u32, u32) {
    let mut range: String = String::new();
    std::io::stdin()
        .read_line(&mut range)
        .expect("Error while trying to read given range");

    let range: Vec<&str> = range.split_terminator(&[' ', ',', '-']).collect();
    let lo: u32 = range[0]
        .trim()
        .parse()
        .expect("Error parsing u32 from string");
    let hi: u32 = range[1]
        .trim()
        .parse()
        .expect("Error parsing u32 from string");

    if lo > hi { (hi, lo) } else { (lo, hi) }
}

fn gen_secret(lo: u32, hi: u32) -> u32 {
    // since we don't have access to packages on the internet (the `rand` crate),
    // we wrote our own little random number generator using bash
    let secret = process::Command::new("randgen.sh")
        .args([format!("{}", lo), format!("{}", hi + 1)])
        .output()
        .expect("Error running external command `randgen.sh`");
    let secret: String =
        String::from_utf8(secret.stdout).expect("Error converting vector output to string");
    let secret: u32 = secret
        .trim()
        .parse()
        .expect("Error parsing u32 from string");

    secret
}

fn get_guess() -> u32 {
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Error reading user guess");

    let guess: u32 = guess.trim().parse().expect("Error parsing u32 from string");

    guess
}
