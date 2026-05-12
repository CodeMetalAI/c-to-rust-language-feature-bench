struct Tag {
    t: u8, // 4 bits unsigned
    r: i8, // 5 bits signed
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb<F>(cb: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    cb(x)
}

fn use_bits(s: &Tag) -> i32 {
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

fn main() -> i32 {
    let mut s = Tag { t: 0, r: 0 };

    s.t = 15;
    s.r = 31;

    if use_bits(&s) != 15 * 100 + 31 {
        return 1;
    }

    {
        let f = f;
        let t: i64 = 1234;

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