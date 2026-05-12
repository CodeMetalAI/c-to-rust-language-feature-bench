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
    let p = &mut i;

    if !ok_mod(&mut i) {
        return 1;
    }
    if !ok_mod(p) {
        return 2;
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci: i32 = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !ok_mod(&mut s3.a) {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = unsafe { std::slice::from_raw_parts_mut(&mut t as *mut _ as *mut u8, std::mem::size_of::<i32>()) };
        bp[0] = bp[0];
        t = t;
        if t != t {
            return 4;
        }
    }

    0
}

fn ok_mod<T>(x: &mut T) -> bool {
    std::mem::size_of_val(x) > 0
}