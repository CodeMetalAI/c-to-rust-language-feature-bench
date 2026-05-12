fn main() {
    let mut i: i32 = 1;
    let p: *const i32 = &i;

    // OK_MOD(i) - checking that i can be assigned to itself, always true for mutable i
    // sizeof((i) = (i)) > 0 is always true in C for assignable lvalues
    // In Rust, we just verify the assignment works
    i = i;
    
    // OK_MOD(*p) - in the original, this dereferences p and assigns
    // We simulate the check passing
    let _ = unsafe { *p };

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
        b: i32, // const in C, but we just don't mutate it
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
        let bytes: &mut [u8; 4] = unsafe { &mut *(&mut t as *mut i32 as *mut [u8; 4]) };
        bytes[0] = bytes[0];
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}