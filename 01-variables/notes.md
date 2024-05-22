# Variables

> Not every word can be used as the name of a variable. Specifically,
> there are _keywords_ in every programming language, reserved for use
> by the language itself, such as `for`, `if`, etc. For a list of Rust's
> reserved keywords, visit
> https://doc.rust-lang.org/stable/book/appendix-01-keywords.html.

## Printing
When printing the value of a variable, the variable name can go inside
the curly brackets. When printing the result of evaluating an expression,
place empty curly brackets in the format string, then follow the format
string with a comma-separated list of expressions to print in each empty
curly bracket placeholder in the same order.
```rust
let a = 10;
println!("{}, {}, {}", a, a * 10, a * 100);
println!("{a}, {}, {}", a * 10, a * 100); // alternative
```

## Mutability
Variables in Rust are immutable by default.
```rust
let x = 42;
x = 43; // gives error
```

You can make a variable mutable by using `mut`.
```rust
let mut y = 13;
y = 7; // all good!
```
This also adds to readability by allowing future readers
of the code to know this value is going to change.

### Shadowing
You can declare a new variable with the same name as a
previous variable; we say that the first variable is _shadowed_
by the second.
```rust
let age = 13;
let age = 23; // shadows previous variable

{
    let age = 33; // shadows outer-scope variable, but only in this scope
    println!("{age}"); // 33
}

println!("{age}"); // 23
```

- Different that using `mut`. Compiler will throw an error if we accidentally try to reassign an immutable variable without using the `let` keyword
-  We're effectively creating a new variable when we use `let`, meaning
we can also change the _type_ of the variable

## Constants
```rust
const PI: f64 = 3.141592;
```

Like variables, constants are values that are bound to a
name and are not allowed to change, but there are differences:

- You can't use `mut` with constants -- they are always immutable
- The type **must be** annotated
- Constants may be set only to a constant expression (see: [Constant evaluation](https://doc.rust-lang.org/reference/const_eval.html)), not the result of a value that could only be computed at runtime
    ```rust
    const SECONDS_IN_DAY: i64 = 24 * 60 * 60; // okay
    const SECRET: i64 = rand::thread_rng().gen_range(1..100); // NOT okay
    ```
- Usually used within the global scope, so that all parts of the program can access it
- Examples include physical or mathematical constants, or the maximum
number of players in a multiplayer session
