fn main() {
    // allocate data on the heap
    let a = Box::new([0; 1_000_000]);
    let b = a;  // now they both point to the same location. `a` has been "moved"
    drop(b);
    // println!("{a}");  // can't do this because `a` has transferred ownership to `b`
}
