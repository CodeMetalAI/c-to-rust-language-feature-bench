fn main() {
    let mut i: i32 = 1;
    let p: *const i32 = &i;

    // OK_MOD(i) - checking that i can be assigned to itself
    // In Rust, this is trivially true for mutable variables
    i = i;
    
    // OK_MOD(*p) - in safe Rust we work with references
    // The original just checks sizeof of assignment expression > 0
    // which is always true for int, so we just proceed

    let a: [i32; 2] = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci: i32 = 3;
    let _ = ci;

    struct S3 {
        a: i32,
    }
    
    let mut s3 = S3 { a: 4 };
    // OK_MOD(s3.a)
    s3.a = s3.a;

    struct S1 {
        a: i32,
        b: i32, // const member in C, but Rust doesn't have const fields the same way
    }
    
    struct S2 {
        s: S1,
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t: i32 = 9;
        // In safe Rust, we can't do byte-level manipulation like in C
        // but we can simulate the self-assignment behavior
        let bytes = t.to_ne_bytes();
        let mut new_bytes = bytes;
        new_bytes[0] = bytes[0];
        t = i32::from_ne_bytes(new_bytes);
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}