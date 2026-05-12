// Define OK_MOD macro
macro_rules! ok_mod {
    ($x:expr) => {{
        let _ = $x;
        std::mem::size_of_val(&$x) > 0
    }};
}

// Define S1 struct
#[derive(Debug)]
struct S1 {
    a: i32,
    b: i32,
}

// Define S2 struct that contains S1
#[derive(Debug)]
struct S2 {
    s: S1,
}

// Define S3 struct
#[derive(Debug)]
struct S3 {
    a: i32,
}

fn main() {
    // Test int
    let mut i = 1;
    let p = &i;
    if !ok_mod(i) {
        std::process::exit(1);
    }
    if !ok_mod(*p) {
        std::process::exit(2);
    }

    // Test array
    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    // Test const
    const CI: i32 = 3;
    let _ = CI;

    // Test S3
    let s3 = S3 { a: 4 };
    if !ok_mod(s3.a) {
        std::process::exit(3);
    }

    // Test S1
    let s1 = S1 { a: 5, b: 6 };
    let mut s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let t = 9;
        let mut bp = t as *const u8 as *mut u8;
        unsafe {
            *bp = *bp;
        }
        let t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}