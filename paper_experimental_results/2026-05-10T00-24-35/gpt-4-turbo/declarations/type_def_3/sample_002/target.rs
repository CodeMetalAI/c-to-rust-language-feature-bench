struct Tag {
    t: u8, // 4 bits, needs a type that can store it; u8 is suitable
    r: i16, // 5 bits as plain, i16 to fit signed 5-bit value
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb<F: Fn(i32) -> i32>(cb: F, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a = s.t as i32;
    let b = s.r as i32;
    a * 100 + b
}

fn f<F: Fn(i32) -> i32>(pf: F) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag {
        t: 15, // only the lower 4 bits are relevant
        r: 31, // only the lower 5 bits are relevant
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

    println!("Success, exiting cleanly.");
}