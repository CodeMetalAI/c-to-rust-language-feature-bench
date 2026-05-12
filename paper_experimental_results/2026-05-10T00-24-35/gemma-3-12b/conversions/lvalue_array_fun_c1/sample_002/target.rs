fn main() {
    let mut i: i32 = 1;
    let p: *mut i32 = &mut i;

    if !std::mem::size_of::<i32>() > 0 {
        return 1;
    }
    if !std::mem::size_of::<i32>() > 0 {
        return 2;
    }

    let a: [i32; 2] = [1, 2];
    let _a = a;
    let _size = std::mem::size_of::<[i32; 2]>();

    const ci: i32 = 3;
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
        let mut t: i32 = 9;
        let bp: *mut u8 = &mut t as *mut i32 as *mut u8;
        unsafe {
            bp[0] = bp[0];
        }
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