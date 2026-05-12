struct Tag {
    t: u8,  // `unsigned t: 4` can be represented by `u8` occupying 4 bits
    r: i8,  // `plain r: 5` and `const t: 5` are signed and can be represented by `i8` 
}

fn id(x: i32) -> i32 { x }

fn call_cb<F>(cb: F, x: i32) -> i32 
where
    F: Fn(i32) -> i32,
{
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t as i32;
    let b = s.r as i32;
    a * 100 + b
}

fn f<F>(pf: F) -> i32 
where
    F: Fn(i32) -> i32,
{
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag {
        t: 15,  // Only lowest 4 bits are used, so this equals to binary 1111 which is decimal 15
        r: 31,  // Only lowest 5 bits are used, so this equals to binary 11111 which is decimal 31
    };

    if use_bits(s) != 15 * 100 + 31 {
        std::process::exit(1);
    }

    {
        let t: i64 = 1234;

        if t != 1234 {
            std::process::exit(2);
        }

        if call_cb(id, 7) != 7 {
            std::process::exit(3);
        }

        if f(id) != 9 {
            std::process::exit(4);
        }
    }
}