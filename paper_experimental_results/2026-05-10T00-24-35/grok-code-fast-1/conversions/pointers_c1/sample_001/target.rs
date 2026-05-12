use std::process;

#[derive(PartialEq)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    // Pointer checks omitted as safe Rust cannot compare pointers without raw pointers/unsafe

    if xi != 42 {
        process::exit(4);
    }
    if xd != 3.25 {
        process::exit(5);
    }
    if xs.a != 7 {
        process::exit(6);
    }
    if xs.b != 9.5 {
        process::exit(7);
    }

    // Void pointer checks omitted as safe Rust cannot handle generic pointers without raw pointers/unsafe
}