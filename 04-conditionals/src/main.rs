/* In Rust, the compiler will throw a warning when
faced with unused objects in the program. To disable
such warnings, you can prefix your variable name with
a `_`. As such, you will be seeing many variables with
`_` behind their name in the next few lessons. */

/* To demonstrate different control flow constructs,
the code in this section is divided into different
functions, each demonstrating a single construct.
Uncomment the function in the body of `main` to see
what it does. */

use std::thread::sleep;
use std::time::Duration;

fn main() {
    // _branching_example(420);
    // _infinite_loop_example();
    // _break_loop_example(27);
    // _label_loop_example();
    // _while_example();
    // _for_example();
    // _for_range_example();
}

fn _branching_example(x: i32) {
    let meaning_of_life = x;

    if meaning_of_life < 42 {
        println!("Heresy!");
    } else if meaning_of_life > 42 {
        println!("May your soul rot in Caelid!");
    } else {
        println!("Thou art an enlightened kind.");
    }

    // you can use expressions in `if` blocks, so the value is returned
    let message = if meaning_of_life == 42 { "Be safe." } else { "Blighted rat!" };
    println!("{message}");
}

fn _infinite_loop_example() {
    // `loop` only stops when interrupted by ^C (Ctrl + C) or `break`
    loop {
        println!("Again!");
        sleep(Duration::from_secs(1));
    }
}

fn _break_loop_example(x: isize) {
    let mut counter = 0;

    let result = loop {
        println!("Current counter value: {}", counter);
        if counter == x {
            break counter;
        }
        counter += 1;
    };

    println!("Result: {result}");
}

fn _label_loop_example() {
    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 7 {
                break;
            }
            if count == 5 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("Final count: {count}");
}

fn _while_example() {
    let mut number = 3;

    while number > 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!")
}

fn _for_example() {
    let names: [&str; 4] = ["Mani", "Armita", "Marina", "Saeid"];
    for name in names {
        println!("My name is {name}");
    }
}

fn _for_range_example() {
    for i in 1..7 {
        println!("{i}");
    }
}
