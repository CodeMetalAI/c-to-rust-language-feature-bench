fn main() {
    let mut i = 1;
    let p = &i;

    if !std::mem::size_of_val(&i) > 0 {
        return 1;
    }
    if !std::mem::size_of_val(p) > 0 {
        return 2;
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let s3 = S3 { a: 4 };
    if std::mem::size_of_val(&s3.a) > 0 {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let t = 9;
        let bp = t as *const u8;
        unsafe {
            *bp = *bp;
        }
        let _ = t;
        if t != t {
            return 4;
        }
    }

    return 0;
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