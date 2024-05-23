/* In Rust, the compiler will throw a warning when
faced with unused variables in the program. To disable
such warnings, you can prefix your variable name with
a `_`. As such, you will be seeing many variables with
`_` behind their name in the next few lessons. */

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
