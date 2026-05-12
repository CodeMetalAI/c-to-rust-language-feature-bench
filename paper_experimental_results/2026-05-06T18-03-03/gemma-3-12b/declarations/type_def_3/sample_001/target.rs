#[derive(Debug)]
struct Tag {
    t: u8,
    constant_t: i8,
    r: u8,
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t as i32;
    let b = s.r as i32;
    a * 100 + b
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag {
        t: 15,
        constant_t: 0,
        r: 31,
    };

    if use_bits(s) != (15 * 100 + 31) as i32 {
        return 1;
    }

    {
        let f = |pf: fn(i32) -> i32| call_cb(pf, 9);
        let mut t: i64 = 1234;

        if t != 1234 {
            return 2;
        }

        if call_cb(id, 7) != 7 {
            return 3;
        }

        if f(id) != 9 {
            return 4;
        }
    }

    0
}