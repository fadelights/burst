* A block is an expression that is allowed to contain statements.
It also defines a syntactic scope for let-bindings inside it.
    ```rust
    fn main() {
        let x = {
            let y = 2;
            y + 1
        };
    }
    ```
* The loop keyword tells Rust to execute a block of code over and over again
forever or until you explicitly tell it to stop.
* The semicolon after `return` or `break` is optional.
* `loop`s can be labeled. This will allow us to break an outer loop from
within an inner loop.
