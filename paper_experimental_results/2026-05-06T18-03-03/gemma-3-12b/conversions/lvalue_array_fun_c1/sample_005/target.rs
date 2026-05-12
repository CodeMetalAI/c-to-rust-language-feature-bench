fn main() {
    let mut i = 1;
    let p = &mut i;

    if !std::mem::size_of::<i>() > 0 {
        return 1;
    }
    if !std::mem::size_of::<i>() > 0 {
        return 2;
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of::<[i32; 2]>();

    let ci = 3;
    let _ = ci;

    let s3 = S3 { a: 4 };
    if !std::mem::size_of::<S3>() > 0 {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: s1 };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = unsafe { &mut (t as *mut i32) as *mut u8 };
        bp[0] = bp[0];
        t = t;
        if t != t {
            return 4;
        }
    }

    0
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