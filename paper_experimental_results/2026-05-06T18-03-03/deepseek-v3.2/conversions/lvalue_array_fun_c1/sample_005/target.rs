fn main() {
    // Define OK_MOD macro equivalent
    macro_rules! OK_MOD {
        ($x:expr) => {
            {
                let mut temp = $x;
                temp = temp;
                std::mem::size_of_val(&temp) > 0
            }
        };
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

    #[derive(Clone, Copy)]
    struct S1 {
        a: i32,
        b: i32,
    }
    
    #[derive(Clone, Copy)]
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
            // This is the only way to replicate the pointer arithmetic behavior
            // while maintaining the same memory access pattern
            *bp = *bp;
        }
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}