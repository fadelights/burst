fn main() {
    let a = [0; 10]; // this array lives on the stack
    let mut b = a; // data is copied here
    b[0] = 1;
    println!("{:?}\n{:?}", a, b); // `a`'s data hasn't changed


    // allocate data on the heap using `Box`
    let a = Box::new([0; 1_000_000]);
    let b = a; // now they both point to the same location. `a` has been "moved"
    drop(b);
    // println!("{:?}", a); // ERROR: can't do this because `a` has transferred ownership to `b`

    // deep copy using `.clone()`
    let a = Box::new([1; 10]);
    let b = a.clone();
    drop(b);
    println!("{:?}", a); // even though `b` is dropped, `a` has maintained ownership of its data

    // referencing in functions
    let name = String::from("Franky");
    greet(&name);

    // referencing and dereferencing
    let v = vec![1, 2, 3];
    let v0 = &v[0];
    let &v1 = &v[1]; // dereferecing upon variable assignment
    println!("{}, {}, {}", v0, *v0, v1); // implicit and explicit dereferencing
}

fn greet(name: &String) {
    println!("Welcome, dear {}!", name);
}
