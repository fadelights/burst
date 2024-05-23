fn main() {
    // numeric literals
    let decimal: i64 = 1_234_789;
    let decimal_suffixed = 1_000_000u64; // pay attention to the suffixed type
    let hex: i32 = 0xffff;
    let octal = 0o77i16; // suffixed type

    println!("Examples of numeric literals printed: ");

    println!("{decimal}");
    println!("{decimal_suffixed}");
    println!("{hex}");
    println!("{octal}\n");

    // floating-point literals
    let double_fp = 2.0;
    let single_fp: f32 = 3.0;
    let single_fp_suffixed = 4.0f32; // suffixed type

    println!("Examples of floating point literals printed: ");

    println!("{double_fp}");
    println!("{single_fp}");
    println!("{single_fp_suffixed}\n");

    // tuples
    let tuple: (i32, f32, char) = (12, 42.3, 'a');
    println!("tuple.0 is {}", tuple.0);

    // tuple deconstruction
    let (x, y, z) = tuple;
    println!("Deconstructed tuple: {}, {}, {}\n", x, y, z);

    // arrays
    let array: [isize; 3] = [12, 72, 89];
    println!("array[1] is {}", array[1]);

    // arrays with repeated value
    let repeated = [0; 300]; // [0, 0, 0, ..., 0]
    println!("The repeated array's length is {}", repeated.len());
}
