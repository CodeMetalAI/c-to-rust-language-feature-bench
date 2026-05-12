fn main() {
    type T = i32;
    type Plain = i32;

    struct Tag {
        t: u8,      // unsigned t : 4
        _const: u8, // const t : 5 (anonymous bitfield)
        r: u8,      // plain r : 5
    }

    fn id(x: T) -> T {
        x
    }

    fn call_cb(cb: fn(T) -> T, x: T) -> T {
        cb(x)
    }

    fn use_bits(s: Tag) -> T {
        let a = s.t as T;
        let b = s.r as T;
        a * 100 + b
    }

    fn f(pf: fn(T) -> T) -> T {
        call_cb(pf, 9)
    }

    let s = Tag {
        t: 15u8,
        _const: 0,
        r: 31u8,
    };

    if use_bits(s) != (15 * 100 + 31) {
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

    std::process::exit(0);
}