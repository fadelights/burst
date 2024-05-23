# Data Types

- Rust is a statically typed language,
- However, the compiler can infer the type of _some_ variables at
compile time if not specified explicitly
- There are two groups of basic data types: scalar and compound

## Scalar
- Numeric values' digits, in general, can be separated with a `_`
to increase readability; e.g. `1_234_000_567` is the same as
`1234000567` (you can also put the separator wherever you like, if
you're weird enough... `12_3456_1` is no different than `1234561`)

### Integer Types
| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

- Signed numbers are stored using two’s complement representation
- Each signed variant can store numbers from
$-(2^{n-1})$ to $2^{n-1} - 1$ inclusive
- Unsigned variants can store numbers from $0$ to $2^n - 1$
- `isize` and `usize` depend on the architecture of the computer your
program is running on -- 64 bits for 64-bit architecture and
32 for 32.
- `i32` is the default

Integer literals can be written in any of the forms below:
| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

When a numeric literal can have multiple types, you may
suffix it with a type, e.g. `57u8`.

- When compiling in debug mode, Rust includes checks for integer
overflow that will cause the program to panic when such behavior
occurs. When compiling in `--release` mode, Rust does **not**
include such checks and as such, wrap around behavior can occur
- Rust provides methods to explicitly handle overflow possibilities

### Floating-Point Types
| Length | Type  |
| ------ | ----- |
| 32-bit | `f32` |
| 64-bit | `f64` |

- All floating-point types are signed
- `f64` is the default

```rust
let x = -32.0; // f64
let y: f32 = 14.0 // f32
```

### Numeric Operations
- Rust supports all the goodies you'd expect from any programming language nowadays
-  Integer division truncates toward zero to the nearest integer

```rust
let x = 32.2 / 12.3; // 2.6178...
let y = -5 / 3; // -1
```

### The Boolean Type
- One byte in size
- `true` or `false`

### The Character Type
- 4 bytes in size
- Uses single quotes `'`
- Represents a Unicode Scalar Value

```rust
let cat = '😺';
```

## Compound
Rust has two primitive compound types: Tuples and Arrays.

If you try to access an invalid array index, the compiler will panic.
This is an example of Rust’s memory safety principles in action.
In many low-level languages, this kind of check is not done,
and when you provide an incorrect index, invalid memory can be accessed.

### The Tuple Type
- General way of grouping together values of **multiple types**
    ```rust
    let tup: (i32, f64, char) = (42, 13.0, '👀');
    ```
- Fixed length
- Elements can be accessed via `.` and an index
    ```rust
    tup.0; // 42
    tup.1; // 13.0
    ```
- Tuples can be destructured
    ```rust
    let (x, y, z) = tup;
    ```
- The tuple without any value the _unit_ tuple;
the value and its type are both written as `()` --
it represents the _empty value_ or _empty return type_
- Expressions implicitly return the unit value
if they don’t return any other value

### The Array Type
- Every element has the **same type**
- The type notation `[i32; 5]` denotes an array with elements of type `i32` and length `5`
    ```rust
    let arr: [f64; 3] = [66.6, 4.2, 0.13];
    ```
- Alternative (concise) initialization notation for repeated initial values
    ```rust
    let zeros = [0; 6]; // [0, 0, 0, 0, 0, 0]
    ```
- Fixed length
- Single chunk of memory with known, fixed size that can be allocated **on the stack**
- Elements accessed via `[]` indexing
    ```rust
    arr[0]; // 66.6
    ```
> If you try to access an invalid array index, Rust will panic and exit.
> This is an example of Rust's memory safety principles.
> Other languages will not panic in such scenarios,
> allowing users to access irrelevant regions of memory.
