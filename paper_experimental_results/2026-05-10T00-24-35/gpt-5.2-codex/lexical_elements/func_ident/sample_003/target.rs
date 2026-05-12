fn streq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = 0usize;
    loop {
        let a_i = if i < a_bytes.len() { a_bytes[i] } else { 0 };
        let b_i = if i < b_bytes.len() { b_bytes[i] } else { 0 };
        if a_i == b_i {
            if a_i == 0 {
                return true;
            }
            i += 1;
        } else {
            return false;
        }
    }
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name("f", "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if !check_name("g", "g") {
        return -2;
    }
    0
}

fn main() {
    let result = if !check_name("main", "main") {
        1
    } else {
        f() + g()
    };
    std::process::exit(result);
}