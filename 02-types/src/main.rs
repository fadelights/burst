fn main() {
    // numeric literals
    let decimal: i64 = 1_000;
    let decimal_suffixed = 1_000_000u64;
    println!("{decimal}");
    println!("{decimal_suffixed}");

    let hex: i32 = 0xffff;
    println!("{hex}");

    let octal = 0o77i16;
    println!("{octal}");

    // prefixing variables with _ disables compiler warnings for unused variables
    let _double_fp = 2.0;
    let _single_fp: f32 = 3.0;
    let _single_fp_suffixed = 4.0f32;

    // compound types
    // tuples
    let tuple: (i32, f32, char) = (12, 42.3, 'a');
    let (_x, _y, _z) = tuple;

    // accessing via indices
    let _x = tuple.0;
    let _y = tuple.1;

    // arrays
    let _array: [isize; 3] = [12, 72, 89];

    // arrays with repeated value
    let _repeated = [0; 3]; // [0, 0, 0]
}
