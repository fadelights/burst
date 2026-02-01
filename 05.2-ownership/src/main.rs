/* Ownership Rules:
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
 */

/* The data in your Rust code are either placed on the stack or the heap:
When you put data on the heap, you request a certain amount of space.
The memory allocator finds an empty spot in the heap that is big enough,
marks it as being in use, and returns a pointer, which is the address of
that location. This process is called "allocating on the heap" and is
sometimes abbreviated as just allocating (pushing values onto the stack
is not considered allocating). Because the pointer to the heap is a known,
fixed size, you can store the pointer on the stack, but when you want the
actual data, you must follow the pointer.
- Pushing to the stack is faster than allocating on the heap because the
  allocator never has to search for a place to store new data; that location
  is always at the top of the stack.
- Accessing data in the heap is generally slower than accessing data on the
  stack because you have to follow a pointer to get there.
 */

#![allow(unused)]
fn main() {
    // string literals (`&str`) are stored on the stack
    // the `String` type's data is allocated on the heap
    let s_stack = "Robert";
    let s_heap = String::from(s_stack);

    // data types that store data on the stack implement the `Copy` trait
    // which results in their data being copied during an assignment operation;
    // no need for a `clone` to perform a "deep copy"
    let mut x = [1, 2, 3];
    let mut y = x; // the entire array is copied
    let z = x.clone(); // essentially the same as above

    // modifying either array does not modify the rest
    x[0] = 1000;
    y[1] = 2000;

    println!("{x:?}");
    println!("{y:?}");
    println!("{z:?}");

    /* Remember that arrays are fixed-sized with their sizes
    being known at compile-time. That is why their data is
    stored on the stack. `Vectors`, on the other hand, are
    dynamiclly-sized collections, which is why their data
    is stored on the heap. */

    /* Since `Strings`s allocate data on the heap, and only
    store the pointer on the stack, they do not implement a
    `Copy` trait so a simple assignment would just "move" the
    ownership of the pointer to a new variable, invalidating
    the pervious variable which owned the pointer.

    The reason behind the "moving" and "ownership" concepts is
    to prevent freeing the heap memory twice when variables go
    out of scope. */
    let s1 = String::from("Marcus");
    let s2 = s1; // s2 now owns the data that s1 points to
    // println!("{s1}"); // ERROR: we cannot use s1 anymore

    let s1 = String::from("Marcus");
    let s2 = s1.clone(); // this is acceptable, since the data is also copied
    println!("{}", s1);

    /* If a type implements the `Copy` trait, variables that use it do not move,
    but rather are trivially copied, making them still valid after assignment
    to another variable. Rust won’t let us annotate a type with `Copy` if the type,
    or any of its parts, has implemented the `Drop` trait. */

    // when you assign a completely new value to an existing variable,
    // Rust will call `drop` and free the original value’s memory immediately
    let mut s = String::from("I don't like this rock...");
    s = String::from("Nevermind, I LOVE this rock!"); // `drop` is called on the previous string's data

    // ownership rules apply to functions as well
    let s = String::from("Watch your ass.");
    takes_ownership(s);
    // println!("{s}"); ERROR: `s` has now transferred ownership to the variable inside the function

    let mut s = String::from("Watch your bass.");
    s = takes_returns_ownership(s);
    println!("{s}"); // OK: since a string was returned, the owner of that string is now `s`

    /* Remember that ownership rules do NOT apply to variables with data on stack.
    These values are simply copied upon assignment. */

    /* Passing variables to functions can quickly become a pain with all these ownerships
    moving around, but Rust has a solution for this: In idiomatic Rust, we don't usually
    pass variables to functions, but rather *references* to variables. */
    let s = String::from("What's a reference?!");
    println!("{}", get_len(&s)); // `s` retains ownership of its data
    println!("{}", get_cap(&s)); // as is evident here in it being used again

    // Note that a reference’s scope starts from where it is introduced
    // and continues through the last time that reference is used.
    let mut a = String::from("I am a silly little string.");
    let b = &mut a;
    println!("{}", b); // `b`'s scope ends here, so the data is now freed and can be used by other mutable references

    let c = &mut a;
    println!("{}", c);

    /* The general rules to follow:
    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid. */

    /* Slices let you reference a contiguous sequence of elements in a collection.
    A slice is a kind of reference, so it does not have ownership. */
    let something = String::from("This is a sentence. که در درون خود فارسی دارد.");
    let first_word = find_first(&something);
    println!("The first word of \"{something}\" is \"{first_word}\"");

    // (mutable) dereferencing during variable creation
    let mut a = [0; 3];
    let b = &mut a;
    let (&mut (mut c)) = b; // a stack copy happens here
    let d = *b; // or just do this...

    c[0] = 10;

    println!("c: {c:?}");
    println!("b: {b:?}");
    println!("a: {a:?}");

    // slice syntax
    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b = &a[1..=3];
    let c = &a[..4];
    let d = &a[5..];
    let e = &a[..];

    // &str is a String slice
    let hello = &String::from("hello, world")[..5];
}

fn takes_ownership(s: String) {
    println!("I own {s} now!");
}

fn takes_returns_ownership(s: String) -> String {
    println!("I own {s} now! But I'm giving it back.");
    s
}

fn get_len(s: &String) -> usize {
    return s.len();
}

fn get_cap(s: &String) -> usize {
    return s.capacity();
}

// f you have a reference to some data,
// the compiler will ensure that the data will not go out of scope
//  before the reference to the data does. Erroneous code example:
/* fn dangle() -> &String {
    let s = String::from("something");
    return &s;
} */


/* The &str type is in fact the "String-slice" type and refers
to a part of a string. Using slice types in function signatures
instead of the actual collection type allows for more robust parameter
handling, as a String is still considered a (full) slice of itself.

Equivalent slice types for arrays are denoted as `&[T]` where T is the
type of elements inside the array. */
fn find_first(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s
}
