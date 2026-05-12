use std::process;

macro_rules! ok_mod {
    ($x:expr) => { true };
}

#[derive(Debug)]
struct S1 {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct S2 {
    s: S1,
}

#[derive(Debug)]
struct S3 {
    a: i32,
}

fn main() {
    let mut i = 1;
    let p = &mut i;

    if !ok_mod!(i) {
        process::exit(1);
    }
    if !ok_mod!(*p) {
        process::exit(2);
    }

    let a: [i32; 2] = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !ok_mod!(s3.a) {
        process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        // No-op in safe Rust equivalent
        if t != t {
            process::exit(4);
        }
    }
}