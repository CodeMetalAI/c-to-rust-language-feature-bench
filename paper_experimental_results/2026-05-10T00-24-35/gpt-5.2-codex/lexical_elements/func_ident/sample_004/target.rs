fn streq(a: &str, b: &str) -> i32 {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    let mut i = 0usize;
    loop {
        let ac = if i < ab.len() { ab[i] } else { 0 };
        let bc = if i < bb.len() { bb[i] } else { 0 };
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
    let status = if check_name("main", "main") == 0 {
        1
    } else {
        f() + g()
    };
    std::process::exit(status);
}