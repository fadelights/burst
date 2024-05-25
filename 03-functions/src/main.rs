fn main() {
    let x = 256;
    let y = square(x);
    let z = cube(x);

    println!("{x} squared is {y}");
    println!("{x} cubed is {z}");
}

fn square(x: i128) -> i128 {
    return x * x
}

fn cube(x: i128) -> i128 {
    /* If you don't specify an explicit return expression,
    the function will implicitly return the last expression's
    result. */
    x * x * x
}
