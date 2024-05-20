> "Safety is the Absence of Undefined Behavior"
- A foundational goal of Rust is to ensure that yourprograms never have undefined behavior.
- A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time.

> "Memory is the space where data is stored during the execution of a program."
- Rust Does Not Permit Manual Memory Management!
- Stacks are automatically managed by Rust.
- Heaps are automatically managed by Rust too! If a variable "owns" some data
on the heap, when the variable goes out of scope (and is deallocated),
then Rust deallocates the relevant heap memory too.
- Assignment switches the ownership of a variable.
    ```rust
    let a = Box::new([0, 1_000_000]); // `a` is the owner
    let b = a; // now `b` is the owner
    ```
- The data on the heap can only be accessed through it's **current** owner.
