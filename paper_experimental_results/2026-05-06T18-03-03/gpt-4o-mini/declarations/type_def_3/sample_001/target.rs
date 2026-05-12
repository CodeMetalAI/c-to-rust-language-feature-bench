type T = i32;
type Plain = i32;

struct Tag {
    t: u8,  // 4 bits
    const_t: u8, // 5 bits
    r: u8,  // 5 bits
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

fn main() -> i32 {
    let s = Tag {
        t: 15,
        const_t: 0, // const_t is not used in the original code
        r: 31,
    };

    if use_bits(s) != (15 * 100 + 31) {
        return 1;
    }

    {
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