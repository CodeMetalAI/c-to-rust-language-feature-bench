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

fn main() {
    let mut i = 1;
    let p = &mut i;

    // OK_MOD equivalents: in Rust, these are assignable
    i = i;
    *p = *p;

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    s3.a = s3.a;

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        // In safe Rust, we can't directly manipulate bytes like this,
        // but the operation is a no-op, so we skip it.
        t = t;
        if t != t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}