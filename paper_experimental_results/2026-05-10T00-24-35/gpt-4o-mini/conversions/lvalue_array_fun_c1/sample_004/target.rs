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

fn ok_mod<T>(_: T) -> bool {
    std::mem::size_of::<T>() > 0
}

fn main() -> i32 {
    let mut i = 1;
    let p = &mut i;

    if !ok_mod(i) {
        return 1;
    }
    if !ok_mod(*p) {
        return 2;
    }

    let a: [i32; 2] = [1, 2];
    let _ = a; // equivalent to (void)a;
    let _ = std::mem::size_of_val(&a); // equivalent to (void)sizeof(a);

    const CI: i32 = 3;
    let _ = CI; // equivalent to (void)ci;

    let s3 = S3 { a: 4 };
    if !ok_mod(s3.a) {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1; // equivalent to (void)s1;
    let _ = s2; // equivalent to (void)s2;

    {
        let mut t = 9;
        let bp = &mut t as *mut i32 as *mut u8;
        unsafe {
            *bp.offset(0) = *bp.offset(0);
        }
        t = t;
        if t != t {
            return 4;
        }
    }

    return 0;
}