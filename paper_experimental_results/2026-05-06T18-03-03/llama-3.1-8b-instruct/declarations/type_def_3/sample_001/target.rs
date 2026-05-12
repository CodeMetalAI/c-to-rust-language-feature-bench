// type_def_3.rs
struct Tag {
    t: u32,
    r: i32,
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: impl Fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t as i32;
    let b = s.r as i32;
    a * 100 + b
}

fn f(pf: impl Fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag {
        t: 15,
        r: 31,
    };

    if use_bits(s) != (15 * 100 + 31) {
        return 1;
    }

    {
        let mut t = 1234;

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

    return 0;
}