macro_rules! OK_MOD {
    ($x:expr) => { true };
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

fn main() {
    let mut i = 1i32;
    let p = &mut i;

    if !OK_MOD!(i) {
        std::process::exit(1);
    }
    if !OK_MOD!(*p) {
        std::process::exit(2);
    }

    let a = [1i32, 2];
    let _ = &a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3i32;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !OK_MOD!(s3.a) {
        std::process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9i32;
        let mut bytes = t.to_ne_bytes();
        bytes[0] = bytes[0];
        t = i32::from_ne_bytes(bytes);
        let _ = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}