fn main() {
    let mut i = 1;
    let p = &mut i;

    // Check if we can modify `i` and `*p`
    if false { // In Rust, the condition to modify a let-bound variable is based on mutability, which is always known at compile time.
        return std::process::exit(1);
    }
    if false {
        return std::process::exit(2);
    }

    let a = [1, 2];
    let _ = &a; // equivalent to `(void)a;`
    let _ = std::mem::size_of_val(&a); // equivalent to `(void)sizeof(a);`

    let ci = 3;
    let _ = &ci; // equivalent to `(void)ci;`

    let mut s3 = S3 { a: 4 };
    if false {
        return std::process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 }};
    let _ = &s1; // equivalent to `(void)s1;`
    let _ = &s2; // equivalent to `(void)s2;`

    {
        let mut t = 9;
        let bp = &mut t as *mut _ as *mut u8;
        unsafe {
            *bp = *bp;
        }
        let _ = &t; // equivalent to `t = t;`
        if t != t {
            return std::process::exit(4);
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