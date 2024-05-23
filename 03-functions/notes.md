# Functions

```rust
fn add_three(x: f64, y: f64, z: f64) -> f64 {
    let result = x + y + z;
    return result; // semicolon is optional in return statements
}
```

- Rust code uses `snake_case` as the conventional style for function and variable names
- The location where you define your function doesn't matter (before or after main)
- Input and output types **must be specified** in the function signature

# Statements and Expressions

Rust is an "expression-based" language:
- _Statements_ are instructions that perform some action and do **not** return a value.
- _Expressions_ evaluate to a resultant value.

Some examples of statements include:
- Variable creation using `let` statements
    - Therefore, you can't assign the result of a `let` statement to another
        ```rust
        let x = (let y = 30); // errors out
        ```
- Function definitions

Some examples of expressions include:
- Numbers and values: `1`, `'c'`, `"Coward!"`, etc.
- Function calls: `sin(30)`, `add(1, 2)`, etc.
- Arithmetic: `1 + 2`, `-3/90`, etc.
- Scope blocks created with curly brackets
    ```rust
    {
        let z = 30;
        z * 60
    }
    ```
    Here, the value of `z * 60` is returned by the block
- Expressions do **not** include ending semicolons. Adding a semicolon
to the end of an expression turns it into a statement
