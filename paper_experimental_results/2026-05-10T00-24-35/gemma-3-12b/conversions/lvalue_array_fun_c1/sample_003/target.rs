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
    let _a = a;
    let _size = std::mem::size_of::<[i32; 2]>();

    let ci = 3;
    let _ci = ci;

    let s3 = S3 { a: 4 };
    if !std::mem::size_of::<S3>() > 0 {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: s1 };
    let _s1 = s1;
    let _s2 = s2;

    {
        let mut t = 9;
        let bp = (t as *mut i32) as *mut u8;
        bp.write(bp.read());
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