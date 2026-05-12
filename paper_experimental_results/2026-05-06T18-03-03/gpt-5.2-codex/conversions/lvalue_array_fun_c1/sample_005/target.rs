fn ok_mod_i32(x: &mut i32) -> bool {
    *x = *x;
    true
}

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

fn run() -> i32 {
    let mut i: i32 = 1;
    if !ok_mod_i32(&mut i) {
        return 1;
    }
    {
        let p: &mut i32 = &mut i;
        if !ok_mod_i32(p) {
            return 2;
        }
    }

    let a: [i32; 2] = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci: i32 = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !ok_mod_i32(&mut s3.a) {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t: i32 = 9;
        let mut bytes = t.to_ne_bytes();
        bytes[0] = bytes[0];
        t = i32::from_ne_bytes(bytes);
        t = t;
        if t != t {
            return 4;
        }
    }

    0
}

fn main() {
    std::process::exit(run());
}