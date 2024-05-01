A block is an expression that is allowed to contain statements.
It also defines a syntactic scope for let-bindings inside it.

```rust
fn main() {
    let x = {
        let y = 2;
        y + 1
    };
}
```
