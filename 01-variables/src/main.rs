fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = 2*x;
    println!("The value of x is: {x}");

    {
        let x = 50;
        println!("The value of [inner-scope] x is: {x}");
    }

    println!("The value of x [outer-scope] is: {x}");
}
