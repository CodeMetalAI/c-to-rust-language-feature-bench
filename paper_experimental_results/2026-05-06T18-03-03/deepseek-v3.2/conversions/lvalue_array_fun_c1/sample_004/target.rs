fn main() {
    // Define OK_MOD macro equivalent
    macro_rules! OK_MOD {
        ($x:expr) => {
            std::mem::size_of_val(&($x = $x)) > 0
        };
    }

    struct S1 {
        a: i32,
        b: i32, // const in C, but Rust doesn't allow const fields in structs
    }

    struct S2 {
        s: S1,
    }

    struct S3 {
        a: i32,
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

    let mut s3 = S3 { a: 4 };
    if !OK_MOD!(s3.a) {
        std::process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = &mut t as *mut i32 as *mut u8;
        unsafe {
            // This is unsafe but required to match C behavior
            // We'll wrap it in unsafe block since raw pointer dereference is needed
            *bp = *bp;
        }
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}