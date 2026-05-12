use std::process::exit;

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

fn ok_mod_i32(x: &mut i32) -> bool {
    *x = *x;
    true
}

fn main() {
    let mut i = 1;
    if !ok_mod_i32(&mut i) {
        exit(1);
    }

    {
        let p: &mut i32 = &mut i;
        if !ok_mod_i32(p) {
            exit(2);
        }
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci: i32 = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !ok_mod_i32(&mut s3.a) {
        exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t: i32 = 9;
        let mut bytes = t.to_le_bytes();
        bytes[0] = bytes[0];
        t = i32::from_le_bytes(bytes);
        t = t;
        if t != t {
            exit(4);
        }
    }

    exit(0);
}