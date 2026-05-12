#![allow(unused_variables)]

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

fn main() -> i32 {
    let mut i = 1;
    let p = &i;

    if !std::mem::size_of_val(&i) > 0 {
        return 1;
    }
    if !std::mem::size_of_val(p) > 0 {
        return 2;
    }

    let a = [1, 2];
    std::mem::drop(a);
    std::mem::size_of_val(&a);

    let ci = 3;
    std::mem::drop(ci);

    let s3 = S3 { a: 4 };
    if std::mem::size_of_val(&s3.a) > 0 {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    std::mem::drop(s1);
    std::mem::drop(s2);

    {
        let mut t = 9;
        let bp = t as *const u8;
        unsafe {
            *bp = *bp;
        }
        t = t;
        if t != t {
            return 4;
        }
    }

    0
}