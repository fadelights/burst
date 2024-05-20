If you try to access an invalid array index, the compiler will panic.
This is an example of Rust’s memory safety principles in action.
In many low-level languages, this kind of check is not done,
and when you provide an incorrect index, invalid memory can be accessed.
