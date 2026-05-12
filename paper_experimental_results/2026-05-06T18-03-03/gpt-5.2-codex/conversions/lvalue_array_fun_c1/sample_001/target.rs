use std::mem;

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

fn ok_mod(x: &mut i32) -> bool {
    let tmp = *x;
    *x = tmp;
    true
}

fn main() {
    let exit_code = {
        let mut i = 1;

        if !ok_mod(&mut i) {
            1
        } else {
            {
                let p = &mut i;
                if !ok_mod(p) {
                    return std::process::exit(2);
                }
            }

            let a = [1, 2];
            let _ = a;
            let _ = mem::size_of_val(&a);

            let ci: i32 = 3;
            let _ = ci;

            let mut s3 = S3 { a: 4 };
            if !ok_mod(&mut s3.a) {
                3
            } else {
                let s1 = S1 { a: 5, b: 6 };
                let s2 = S2 { s: S1 { a: 7, b: 8 } };
                let _ = s1;
                let _ = s2;

                {
                    let mut t: i32 = 9;
                    let mut bytes = t.to_ne_bytes();
                    bytes[0] = bytes[0];
                    t = i32::from_ne_bytes(bytes);
                    if t != t {
                        4
                    } else {
                        0
                    }
                }
            }
        }
    };

    std::process::exit(exit_code);
}