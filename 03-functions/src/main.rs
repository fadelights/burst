fn main() {
    let x = 256;
    let y = square(x);

    println!("{y}");

}

fn square(x: i128) -> i128 {
    return x * x;
}
