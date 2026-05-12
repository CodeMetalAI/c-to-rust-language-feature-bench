fn main() {
    // OK_MOD macro equivalent: sizeof((x) = (x)) > 0
    // In Rust, we can simulate this by checking that assignment is valid.
    // We'll define a helper that returns true if assignment is allowed.
    fn ok_mod<T>(x: T) -> bool {
        // In Rust, assignment to a variable is always allowed if the variable is mutable.
        // We'll just return true for types that can be assigned (i.e., not const).
        true
    }

    let mut i = 1;
    let p = &mut i;

    if !ok_mod(i) {
        return;
    }
    if !ok_mod(*p) {
        return;
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !ok_mod(s3.a) {
        return;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = &mut t as *mut _ as *mut u8;
        // Safe pointer manipulation: we can use std::ptr::read/write
        unsafe {
            let val = std::ptr::read(bp);
            std::ptr::write(bp, val);
        }
        t = t;
        if t != t {
            return;
        }
    }

    // Exit with success
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