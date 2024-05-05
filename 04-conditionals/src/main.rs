use std::thread::sleep;
use std::time::Duration;


fn main() {
    branches(420);
    // _infinite_loop();
    break_loop(27);
    label_loop();
    while_loop();
    for_loop();
    for_loop_range();
}


fn branches(x: i32) {
    let meaning = x;

    if meaning < 42 {
        println!("Heresy!");
    } else if meaning > 42 {
        println!("Rot in Caelid!");
    } else {
        println!("You may rest in peace.");
    }

    // you can use expressions in `if` blocks, so the value is returned
    let message = if meaning == 42 { "Passable." } else { "Heresy!" };
    println!("{message}");
}


fn _infinite_loop() {
    // only stops when interrupted by ^C (Ctrl + C) or `break`
    loop {
        println!("Again!");
        sleep(Duration::from_secs(1));
    }
}


fn break_loop(x: isize) {
    let mut counter = 0;
    let result = loop {
        if counter == x {
            break counter;
        }

        counter += 1;
    };

    println!("Result: {result}");
}


fn label_loop() {
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


fn while_loop() {
    let mut number = 3;

    while number > 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!")
}


fn for_loop() {
    let names: [&str; 4] = ["Mani", "Armita", "Marina", "Saeid"];
    for name in names {
        println!("My name is {name}");
    }
}


fn for_loop_range() {
    for i in 1..7 {
        println!("{i}");
    }
}
