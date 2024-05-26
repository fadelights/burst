fn main() {
    // allocate data on the heap using `Box`
    let a = Box::new([0; 1_000_000]);
    let b = a; // now they both point to the same location. `a` has been "moved"
    drop(b);

    /* Using {:?} in a format string allows us to
    print an array. */
    // println!("{:?}", a); // can't do this because `a` has transferred ownership to `b`

    // deep copy using `.clone()`
    let a = Box::new([1; 10]);
    let b = a.clone();
    drop(b);
    println!("{:?}", a); // even though `b` is dropped, `a` has maintained ownership of its data

    // referencing and dereferencing
    let v = vec![1, 2, 3];
    let v0 = &v[0];
    println!("{}, {}", v0, *v0); // implicit and explicit dereferencing
}
