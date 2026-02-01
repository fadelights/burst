# Ownership

Ownership is rust's most unique feature and enables Rust
to make memory saftey guarantees, without needing a garbage collector.

## But What Is ‘Safety’?
> _Safety is the absence of undefined behavior._
- A foundational goal of Rust is to ensure that your programs never have undefined behavior
    - Undefined behavior is especially dangerous for low-level programs with direct access to memory
- A secondary goal of Rust is to prevent undefined behavior at _compile-time_ instead of _run-time_
    - Avoids bugs in production, improving the **reliability**
    - Fewer runtime checks for bugs, improving the **performance**
- An error like Python's `NameError` comes at a cost: Each time an interpreted program reads a variable, then the interpreter must check whether that variable is defined
- A list of "behavior considered undefined": https://doc.rust-lang.org/reference/behavior-considered-undefined.html
### Case Study: The Morris Worm
https://en.wikipedia.org/wiki/Morris_worm

- One of the key vulnerabilities the worm exploited was a buffer overflow in the `gets()` function in the Unix operating system.
- The `gets()` function was used to read input without checking the length of the input string, allowing more data to be written into the buffer than it could hold.

## Memory Safety
> _Memory is the space where data is stored during the execution of a program._

### Stack
- Variables live in _frames_. A frame is a mapping from variables to values within a single scope, such as a function
- After a function returns, Rust _deallocates_ (_frees_, or _drops_) the function's frame
- The most recent frame added is always the next frame freed (hence the name, _stack_)
- When an expression reads a variable, the variable's value is copied from its slot in the stack frame
    ```rust
    let a = 5; // L1
    let mut b = a; // L2
    b += 1; // L3
    ```
    ![Copying in the stack](media/stack-copying.png)
    - However, copying data can take a lot of memory (imagine `a` being an array with a million elements)

### Heap
- To transfer access to data without copying it, Rust uses _pointers_. The value that a pointer points-to is called its _pointee_
- One common way to make a pointer is to allocate memory in the _heap_
- The heap is a separate region of memory where data can live indefinitely
- Rust provides a construct called `Box` for putting data on the heap
    ```rust
    let a = Box::new([0; 1_000_000]); // L1 -- `a` is the owner
    let b = a; // L2 -- now `b` is the owner (`a` has been moved)
    ```
    ![Copying in the heap](media/heap-copying.png)

### Rust Does Not Permit Manual Memory Management!
> _Memory management is the process of allocating and deallocating memory._
- Stacks are automatically managed by Rust
- Heaps are automatically managed by Rust too! If a variable "owns" some data on the heap, when the variable goes out of scope (and is deallocated), then Rust deallocates the relevant heap memory too
- Manual memory management can easily lead to bugs. Suppose we have a `free` function in Rust, similar to C
    ```rust
    let a = Box::new([0; 100]);
    free(a);
    println!("{}", a[0]); // undefined behavior
    ```
- Assignment switches the ownership of a variable. In the statement `let b = a;`, we say that `a` has been _moved_
- The data on the heap can only be accessed through it's **current** owner. This is to prevent undefined behavior
    ```rust
    let empyrean = String::from("Malenia");
    let godess = empyrean; // `godess` is now the owner, but `empyrean` still points to "Malenia"
    godess.push_str(", Godess of Rot"); // new location allocated on the heap, the previous location is invalidated, so `empyrean` now points to an invalid memory location
    println!("{empyrean}"); // undefined behavior; therefore prevented by Rust
    ```
- The `.clone()` method, if implemented for a data structure, can do a _deep copy_, allowing a variable to maintain the ownership of its data during an assignment operation
    ```rust
    let first = String::from("Ferris");
    let first_clone = first.clone();
    ```
    ![Cloning](media/clone.png)

## References and Borrowing
Using `clone`s can quickly become tedious.
Say a function takes 2 arguments, then we would have to do 2 `clone`
calls.

### References, the Non-owning Pointers
The expression `&x` is used to create a reference to a variable named `x`.
In Rust jargon, the variable `x` is _borrowed_.

<!-- TODO: Add visualization here -->

```rust
let name = String::from("Lucius");
let nickname = &name;
```

In the example above, `nickname` does **not** own either
`name` or `"Lucius"`. Because `nickname` doesn't own them, the
originial variable (`name`) or its value (`"Lucius"`) will not be
deallocated after the deallocation of the reference variable.

> _References are non-owning pointers._

The `*` operator is used for _dereferencing_.

```rust
let x = 10;
let y = &x; // value of `y` is the address of `x`
            // value of `*y` is 10
```

However, Rust implicitly inserts dereferences and references
in certain cases, such as calling a method with the dot operator.

```rust
let x = Box::new(1);
let p = &x; // `p` is a reference to `x`
println("{}, {}", p, *p); // will print `1, 1` -- p is implicitly dereferenced
```

- This implicit conversion works for multiple layers of pointers
- This conversion also works the opposite direction. The function `str::len` expects a reference `&str`. If you call `len` on an owned `String`, then Rust will insert a single borrowing operator

## Aliasing and Mutation
- Pointers are dangerous because they allow _aliasing_ -- accessing the same data through different variables
- Aliasing is harmless on its own, but combined with _mutation_ is a recipe for disaster
    - The aliased data may be deallocated by another variable
    - Concurrently mutating data may cause a data race

As an example, the `vec!` macro creates a vector on the heap. This
vector can be modified to add or remove elements, but in the process,
it might deallocate the current memory location occupied, and allocate
new memory elsewhere.

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
v.push(4);
println!("Third element is {}", *num);
```
![Dangers of aliasing](media/aliasing-danger.png)

The issue is that the vector `v` is both aliased
(by the reference `num`) and mutated (by the operation `v.push(4)`).

> **Rust's Pointer Safety Principle:**
> _Data should never be aliased and mutated at the same time._

 Rust enforces this principle for boxes (owned pointers) by disallowing
 aliasing. Assigning a box from one variable to another will move
 ownership, invalidating the previous variable. Owned data can only be
 accessed through the owner -- no aliases.

However, because references are non-owning pointers, they need different
rules than boxes to ensure the _Pointer Safety Principle_.

## The Borrow Checker
The core idea: Variables have three kinds of **permissions** on
their data:
1. Read (**R**): data can be copied to another location
2. Write (**W**): data can be mutated in-place
3. Own (**O**): data can be moved or dropped

> _These permissions don't exist at runtime, only within the compiler._

By default, a variable only has **RO** permissons. If it is defined with `mut`, it also has **W**

> _Key idea: References can temporarily remove these permissions._

An example, here is the permissions each variable gains/loses
on its data **after** each line of the code:

```rust
let mut v: Vec<i32> = vec![1, 2, 3];
/* v: +R, +W, +O (RWO) */

let num: &i32 = &v[2];
/* (Data is borrowed by `num`)
v   :  R, -W, -O (R--)
num : +R,   , +O (R-O)  (NOTE: `num` does not own the data of the vector, only the address which it holds!)
*num: +R         (R--)
*/

println!("Third element is {}", *num);
/* (`num` is no longer used, so its permissions are removed)
v   :  R, +W, +O (RWO)
num : -R,   , -O (---)
*num: -R         (---)
*/

v.push(4);
/* (After reaching the end of scope)
v: -R, -W, -O (---)
*/
```

- In simple terms: _Creating a reference to data ("borrowing" it) causes that data to be temporarily read-only until the reference is no longer used_
- The borrow checker looks for potentially unsafe operations involving references
- Each operation on the data of a variable requires some set pf permissions; e.g. `push` requires **RW**. If the variable does not have the necessary permissions, the borrow checker will throw an error

### Mutable References
- Can modify the data they point to
- Denoted with `&mut`
    ```rust
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2]; // `num` will be a mutable reference
    *num += 1; // v = [1, 2, 4]
    ```
- When `num` was an immutable reference, `v` still had the **R** permission. Now that `num` is a mutable reference, `v` has lost **all** permissions while `num` is in use
- When `num` was an immutable reference, the path `*num` only had the **R** permission. Now that `num` is a mutable reference, `*num` has also gained the **W** permission
- Mutable references can also be temporarily "downgraded" to read-only references
    ```rust
    ...
    let num: &mut i32 = &mut v[2];
    /*
    num: +R, +W, (RW-)
    */

    let num2: &i32 = &*num;
    /*
    num :   , -W, (RW-)
    num2: +R,   , (R--)
    */
    ```
### Data Lifespan
As part of the _Pointer Safety Principle_:
> _Data must outlive any references to it._

This property is enforced in two ways:
1. For data **inside** the funcion body, when data is borrowed, ownership is temporarily dropped -- hence preventing the use of `drop` (which requires ownership permission)
1. For **input and output** parameters of a function, which the lifespan of a reference may be ambiguous, a new kind of permission is introduced: The _Flow_ permission; these will be discussed in later chapters. For now, just know that the error message `missing lifetime specifier` means an operation may be missing sufficient flow permissions

## Summary
- References provide the ability to read and write data without consuming ownership of it
- References are created with borrows (`&` and `&mut`) and used with dereferences (`*`), often implicitly
- Rust's borrow checker enforces a system of permissions that ensures references are used safely:
    - All variables can read, own, and (optionally) write their data
    - Creating a reference will transfer permissions from the borrowed **path** to the reference
    - Permissions are returned once the reference's lifetime has ended
    - Data must outlive all references that point to it
- This system allows Rust to not need a garbage collector (increasing speed), while also avoiding undefined behavior (increasing safety)

Refer to the Rust book's chapter on
[Fixing Ownership Errors](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html)
for a few example case studies.
