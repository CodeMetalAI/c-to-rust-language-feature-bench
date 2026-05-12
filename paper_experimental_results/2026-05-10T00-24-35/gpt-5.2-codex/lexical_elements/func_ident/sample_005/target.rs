fn streq(a: &str, b: &str) -> i32 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = 0usize;
    loop {
        let ac = if i < a_bytes.len() { a_bytes[i] } else { 0 };
        let bc = if i < b_bytes.len() { b_bytes[i] } else { 0 };
        if ac == bc {
            if ac == 0 {
                return 1;
            }
            i += 1;
        } else {
            return 0;
        }
    }
}

fn check_name(got: &str, expect: &str) -> i32 {
    streq(got, expect)
}

fn f() -> i32 {
    if check_name("f", "f") == 0 {
        return -1;
    }
    0
}

fn g() -> i32 {
    if check_name("g", "g") == 0 {
        return -2;
    }
    0
}

fn main() {
    let code = if check_name("main", "main") == 0 {
        1
    } else {
        f() + g()
    };
    std::process::exit(code);
}