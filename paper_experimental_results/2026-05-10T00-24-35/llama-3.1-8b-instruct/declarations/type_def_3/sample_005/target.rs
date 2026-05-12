// type_def_3.rs

#[derive(Debug)]
struct Tag {
    t: u8,
    const_field: u8,
    r: u8,
}

fn id(x: i32) -> i32 {
    x
}

fn call_cb(cb: impl Fn(i32) -> i32, x: i32) -> i32 {
    cb(x)
}

fn use_bits(s: Tag) -> i32 {
    let a: i32 = s.t as i32;
    let b: i32 = s.r as i32;
    (a * 100 + b)
}

fn f(pf: impl Fn(i32) -> i32) -> i32 {
    call_cb(pf, 9)
}

fn main() {
    let mut s = Tag {
        t: 15,
        const_field: 0, // This field is not used and can be removed
        r: 31,
    };

    if use_bits(s) != (15 * 100 + 31) {
        println!("use_bits(s) failed");
        return;
    }

    {
        let pf: impl Fn(i32) -> i32 = id;

        let t: i64 = 1234;

        if t != 1234 {
            println!("t failed");
            return;
        }

        if call_cb(pf, 7) != 7 {
            println!("call_cb failed");
            return;
        }

        if f(pf) != 9 {
            println!("f failed");
            return;
        }
    }

    println!("All tests passed");
}