struct Tag {
    t: u32,  // unsigned t : 4
    _const_t: u32,  // const t : 5 (unnamed const field)
    r: u32,  // plain r : 5
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = (s.t & 0xF) as i32;  // 4 bits
    let b = (s.r & 0x1F) as i32; // 5 bits
    a * 100 + b
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let s = Tag {
        t: 15u32,
        _const_t: 0,
        r: 31u32,
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