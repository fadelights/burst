#[allow(unused)]
fn main() {
    // immutable array
    let a = [0, 1, 2, 3, 4, 5, 6, 7 ,8, 9];

    let b1 = &a[2..4]; // immutable slice, with `b1` being an immutable pointer to that immutable slice
    let mut b2 = &a[2..4]; // immutable slice, with `b2` being a mutable pointer to that immutable slice

    println!("a: {:?}", a);
    println!("b1: {:?}", b1);
    println!("b2: {:?}", b2);

    // either way, you can't change the contents of the slice, the following code will panic
    // b2[0] = 200;

    // but you CAN change b2 to point somwhere else, since it is a mutable pointer
    b2 = &a[7..];
    println!("new b2: {:?}", b2);

    // you can also create mutable references
    // this requires the original array to be mutable in the first place
    // let c1 = &mut a[2..4]; // ERROR
    let mut a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // doesn't matter if the pointer variable is mutable or immutable,
    // you can modify a mutable array slice, which will also modify the
    // original array
    let c1 = &mut a[2..4];
    // let mut c2 = &mut a[4..6]; // ERROR: rust won't allow this since a mutable borrow has already occurred above that modifies `a`

    c1[0] = 200;

    // even the order of operations matters here
    // if we were to print `a` before `c1`, an error would be raised
    println!("c1: {:?}", c1);
    println!("a: {:?}", a);

    // you can still borrow immutable slices from a mutable array
    let c3 = &a[2..4];
    let mut c4 = & a[2..4];
    // c4[1] = 300; // ERROR: trying to modify an immutable reference

    println!("c3: {:?}", c3);

    // if you want to modify a slice without modifying the original array,
    // you can create a copy of it
    let mut d = [0; 10];
    d[2..4].copy_from_slice(&a[2..4]);
    d[2] = 20000; // no problem

    println!("a: {:?}", a);
    println!("d: {:?}", d);
}
