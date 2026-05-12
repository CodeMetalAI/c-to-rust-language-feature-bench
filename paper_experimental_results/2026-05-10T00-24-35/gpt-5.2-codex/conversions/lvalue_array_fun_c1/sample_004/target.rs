use std::process::exit;

#[derive(Clone, Copy)]
struct S1 {
    a: i32,
    b: i32,
}
#[derive(Clone, Copy)]
struct S2 {
    s: S1,
}
#[derive(Clone, Copy)]
struct S3 {
    a: i32,
}

fn ok_mod<T>(_x: &mut T) -> bool {
    true
}

fn main() {
    let mut i: i32 = 1;
    let mut p: &mut i32 = &mut i;

    if !ok_mod(&mut i) {
        exit(1);
    }
    if !ok_mod(&mut *p) {
        exit(2);
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci: i32 = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !ok_mod(&mut s3.a) {
        exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t: i32 = 9;
        let b0 = t.to_ne_bytes()[0];
        let _ = b0;
        t = t;
        if t != t {
            exit(4);
        }
    }

    exit(0);
}