// Define a function to check if an lvalue is modifiable
fn is_modifiable<T: Copy>(x: &T) -> bool {
    std::mem::size_of_val(x) > 0
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

fn main() {
    let mut i = 1;

    if !is_modifiable(&i) {
        std::process::exit(101);
    }

    let p = &i;

    if !is_modifiable(p) {
        std::process::exit(102);
    }

    let a = [1, 2];
    (_a: a); // ignore the result of sizeof
    std::mem::size_of::<a>();

    let ci = 3;
    (_ci: ci); // ignore the result of sizeof

    let s3 = S3 { a: 4 };
    if !is_modifiable(&s3.a) {
        std::process::exit(103);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    (_s1: s1); // ignore the result of sizeof
    (_s2: s2); // ignore the result of sizeof

    {
        let t = 9;
        let bp: *const u8 = &t as *const u8;
        let _ = bp[0];
        let _ = bp[0];
        let t = t;
        if t != t {
            std::process::exit(104);
        }
    }

    std::process::exit(0);
}