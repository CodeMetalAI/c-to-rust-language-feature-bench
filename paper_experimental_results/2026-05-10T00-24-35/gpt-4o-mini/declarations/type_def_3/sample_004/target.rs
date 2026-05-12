type T = i32;
type Plain = i32;

struct Tag {
    t: u32, // 4 bits
    const_t: T, // 5 bits, const is not used as a keyword in Rust
    r: Plain, // 5 bits
}

fn id(x: T) -> T {
    x
}

fn call_cb(cb: fn(T) -> T, x: T) -> T {
    cb(x)
}

fn use_bits(s: Tag) -> T {
    let a: T = s.t as T;
    let b: T = s.r as T;
    a * 100 + b
}

fn f(pf: fn(T) -> T) -> T {
    call_cb(pf, 9)
}

fn main() -> i32 {
    let s = Tag {
        t: 15,
        const_t: 31, // Rust doesn't have the const storage class, but we'll keep the value
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