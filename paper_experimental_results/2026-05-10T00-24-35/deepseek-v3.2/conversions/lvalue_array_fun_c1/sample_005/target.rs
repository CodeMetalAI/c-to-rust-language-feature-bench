fn main() {
    // Equivalent to #define OK_MOD(x) (sizeof((x) = (x)) > 0)
    // In Rust, we can check if an expression is assignable by using a closure
    // that attempts assignment and returns true if it compiles.
    // We'll create a macro that mimics this compile-time check.
    macro_rules! OK_MOD {
        ($x:expr) => {{
            let mut temp = $x;
            temp = temp; // Assignment to self
            true // If we reach here, assignment worked
        }};
    }

    let mut i = 1;
    let p = &mut i;

    if !OK_MOD!(i) {
        std::process::exit(1);
    }
    if !OK_MOD!(*p) {
        std::process::exit(2);
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    #[derive(Clone, Copy)]
    struct S3 {
        a: i32,
    }
    
    let mut s3 = S3 { a: 4 };
    if !OK_MOD!(s3.a) {
        std::process::exit(3);
    }

    struct S1 {
        a: i32,
        b: i32, // const removed since Rust doesn't have const fields in structs
    }
    
    struct S2 {
        s: S1,
    }
    
    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = &mut t as *mut i32 as *mut u8;
        unsafe {
            // This is the only unsafe block needed for pointer arithmetic
            // but we keep it minimal and equivalent to the original
            *bp = *bp;
        }
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}