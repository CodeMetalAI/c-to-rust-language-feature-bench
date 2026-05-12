fn main() {
    let i = 1;
    let p = &i;

    if!std::ptr::write_volatile(p, *p) {
        return 1;
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if!std::ptr::write_volatile(&mut s3.a, s3.a) {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = t as *mut i32 as *mut u8;
        unsafe { *bp = *bp };
        t = t;
        if t!= t {
            return 4;
        }
    }

    return 0;
}

#[derive(Debug)]
struct S1 {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct S2 {
    s: S1,
}

#[derive(Debug)]
struct S3 {
    a: i32,
}