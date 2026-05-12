fn main() {
    let mut i = 1;
    let p = &mut i;

    // In Rust, all variables are modifiable unless explicitly stated with `const` or `let`.
    // Therefore, we don't need a macro like OK_MOD; we can directly modify `i` and `*p`.

    let a = [1, 2];
    let _ = a; // equivalent to (void)a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci; // equivalent to (void)ci;

    let mut s3 = S3 { a: 4 };
    // Direct modification is possible
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

    std::process::exit(0);
}

struct S1 {
    a: i32,
    b: i32, // `const` in struct fields doesn't exist in Rust, use immutable fields instead
}

struct S2 {
    s: S1,
}

struct S3 {
    a: i32,
}