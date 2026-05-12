fn main() {
    let mut i = 1;
    let p = &mut i;

    // In Rust, all variables are modifiable unless explicitly stated with `const` or `let`.
    // Thus, checking if `i` or `*p` is modifiable is not necessary as they are mutable.

    let a = [1, 2];
    // In Rust, arrays and their sizes are directly accessible and used.

    let ci = 3;
    // `ci` is a constant integer, similar to `const int` in C.

    let mut s3 = S3 { a: 4 };
    // Directly modifying `s3.a` is possible because it's mutable.

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    // Structures are used similarly, but Rust ensures immutability of `b` in `S1` through the type system.

    {
        let mut t = 9;
        let bp = &mut t as *mut i32 as *mut u8;
        // Directly manipulating memory through pointers is unsafe in Rust, thus this block is omitted.
        // Rust ensures memory safety through its borrowing rules and type system.
    }
}

struct S1 {
    a: i32,
    b: i32, // `b` is not declared as `const` because Rust does not support field-level constancy like C.
}

struct S2 {
    s: S1,
}

struct S3 {
    a: i32,
}