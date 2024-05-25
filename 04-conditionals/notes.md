# Branching

## `if`, `else if`, and `else`
- Blocks of code associated with the conditions in `if` expressions are sometimes called arms
- Branching constructs in Rust **only** accept booleans; no funny business such as _truthy_ or _falsey_ values
- `if` blocks are also expressions
    ```rust
    // assuming a variable named `gender` has already been defined...
    let name = if gender == "boy" { "Wōden" } else { "Frīg" };
    ```
    - The values that have the potential to be results from each arm of the `if` must be the same type

## `loop`
- The `loop` construct executes a body of code until either interrupted by the user or using the `break` statement
- An example of using `loop` is checking whether a thread has completed its job
- A `loop` can also return a value, by adding it after `break`
    - The semicolon at the end of `break` is optional (similar to `return`)
- You can label loops to `break` and `continue` nested loops -- if not labeled, the `break` and `continue` will belong to the most inner loop
    ```rust
    'outer: loop {
        // stuff...
        'inner: loop {
            // more stuff...
            break 'outer;
        }
    }
    ```
-

## `while`
Not much to say here... this is also a thing.

## `for`
`for` is best used when iterating over values of a collection, such
as an array or vector.
