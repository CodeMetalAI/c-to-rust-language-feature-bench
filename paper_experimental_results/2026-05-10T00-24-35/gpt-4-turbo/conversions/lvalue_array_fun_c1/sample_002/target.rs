fn main() {
    let mut i = 1;
    let p = &mut i;

    // Direct assignments in conditions are not idiomatic in Rust, so we perform the check after assignment.
    i = i; // simulate the macro OK_MOD
    if i != i {
        return;
    }

    *p = *p; // simulate the macro OK_MOD
    if *p != *p {
        std::process::exit(2);
    }

    let a = [1, 2];
    // In Rust, there's no direct equivalent of C's sizeof, but we can get the length of the array.
    let _ = a;
    let _ = a.len();

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    s3.a = s3.a; // simulate the macro OK_MOD
    if s3.a != s3.a {
        std::process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = &mut t as *mut _ as *mut u8;
        unsafe {
            *bp = *bp;
        }
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
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