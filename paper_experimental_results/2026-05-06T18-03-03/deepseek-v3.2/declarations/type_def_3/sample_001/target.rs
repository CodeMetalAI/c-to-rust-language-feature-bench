struct Tag {
    t: u8, // unsigned t : 4 -> stored as u8, but we'll mask bits
    r: u8, // plain r : 5 -> stored as u8, but we'll mask bits
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = (s.t & 0xF) as i32; // mask 4 bits
    let b = (s.r & 0x1F) as i32; // mask 5 bits
    a * 100 + b
}

fn f(pf: fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let s = Tag {
        t: 15u, // 15 fits in 4 bits
        r: 31,  // 31 fits in 5 bits
    };

    if use_bits(s) != 15 * 100 + 31 {
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