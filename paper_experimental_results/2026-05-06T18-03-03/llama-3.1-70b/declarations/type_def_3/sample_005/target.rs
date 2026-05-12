fn main() {
    #[repr(C, packed)]
    struct Tag {
        t: u8,
        _pad: u8,
        r: i8,
    }

    let mut s = Tag { t: 15, _pad: 0, r: 31 };

    if use_bits(s) != (15 * 100 + 31) as i32 {
        return;
    }

    {
        let t: i64 = 1234;

        if t != 1234 {
            return;
        }

        if call_cb(id, 7) != 7 {
            return;
        }

        if f(id) != 9 {
            return;
        }
    }
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