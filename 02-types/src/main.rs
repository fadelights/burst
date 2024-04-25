fn main() {
    // numeric literals
    let decimal: i64 = 1_000;
    let deciaml_suffixed = 1_000_000u64;
    println!("{decimal}");
    println!("{deciaml_suffixed}");

    let hex: i32 = 0xffff;
    println!("{hex}");

    let octal = 0o77i16;
    println!("{octal}");

    // prefixing variables with _ disables compiler warnings for unused variables
    let _double_fp = 2.0;
    let _single_fp: f32 = 3.0;
    let _single_fp_suffixed = 4.0f32;
}
