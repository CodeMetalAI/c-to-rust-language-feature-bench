fn main() {
    let mut i = 1;
    let p = &mut i;

    // In Rust, we can mutate through mutable references
    // and directly mutable variables, but not through immutable bindings or constants.
    *p = *p;

    let mut a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    s3.a = s3.a;

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = &mut t as *mut i32 as *mut u8;
        unsafe {
            *bp = *bp;
        }
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }
}

struct S1 {
    a: i32,
    b: i32,
}

struct S2 {
    s: S1,
}

struct S3 {
    a: i32,
}