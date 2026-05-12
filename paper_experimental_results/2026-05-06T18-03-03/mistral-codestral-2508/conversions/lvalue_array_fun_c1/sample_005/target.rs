fn main() {
    let mut i = 1;
    let p = &mut i;

    if !is_modifiable(&mut i) {
        std::process::exit(1);
    }
    if !is_modifiable(p) {
        std::process::exit(2);
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if !is_modifiable(&mut s3.a) {
        std::process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = unsafe { std::mem::transmute::<&mut i32, &mut [u8; 4]>(&mut t) };
        bp[0] = bp[0];
        let _ = t;
        if t != t {
            std::process::exit(4);
        }
    }
}

fn is_modifiable<T>(_: &mut T) -> bool {
    true
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