fn main() {
    let mut i = 1;
    let p = &mut i;

    if !std::mem::size_of_val(&i) > 0 {
        std::process::exit(1);
    }
    if !std::mem::size_of_val(&*p) > 0 {
        std::process::exit(2);
    }

    let a = [1, 2];
    let _ = &a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !std::mem::size_of_val(&s3.a) > 0 {
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
            *bp = *bp; // simulate a byte-wise self-assignment of `t`
        }
        t = t; // self-assignment of `t`
        if t != t { // always false, but simulating condition check
            std::process::exit(4);
        }
    }

    std::process::exit(0);
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