#[derive(Clone)]
struct S1 {
    a: i32,
    b: i32,
}

#[derive(Clone)]
struct S2 {
    s: S1,
}

#[derive(Clone)]
struct S3 {
    a: i32,
}

fn main() {
    let mut i = 1;
    i = i;

    let mut ii = 1;
    let p = &mut ii;
    *p = *p;

    let _a = [1, 2];
    let _ci = 3;

    let mut s3 = S3 { a: 4 };
    s3.a = s3.a;

    let _s1 = S1 { a: 5, b: 6 };
    let _s2 = S2 { s: S1 { a: 7, b: 8 } };

    {
        let t = 9;
        let t_val = t;
        if t != t_val {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}