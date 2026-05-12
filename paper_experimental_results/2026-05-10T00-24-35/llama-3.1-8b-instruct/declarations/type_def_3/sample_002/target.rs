// type_def_3.rs

struct Tag {
    t: u4,
    const_t: u5,
    r: u5,
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: impl Fn(i32) -> i32, x: i32) -> i32 {
    (cb)(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t as i32;
    let b = s.r as i32;
    a * 100 + b
}

fn main() {
    let mut s = Tag {
        t: 15u4,
        const_t: 0u5,
        r: 31u5,
    };

    if use_bits(s) != 15 * 100 + 31 {
        panic!();
    }

    {
        let f: impl Fn(i32) -> i32 = id;
        let mut t: i32 = 1234;

        if t != 1234 {
            panic!();
        }

        if call_cb(id, 7) != 7 {
            panic!();
        }

        if f(id) != 9 {
            panic!();
        }
    }

    let f = |x: i32| -> i32 { call_cb(id, 9) };
    assert_eq!(f(id), 9);
}