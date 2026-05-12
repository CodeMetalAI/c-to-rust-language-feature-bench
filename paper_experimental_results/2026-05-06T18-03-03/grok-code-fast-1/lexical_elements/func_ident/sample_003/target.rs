fn streq(a: &str, b: &str) -> i32 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = 0;
    while i < a_bytes.len() && i < b_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return 0;
        }
        i += 1;
    }
    if a_bytes.len() == b_bytes.len() { 1 } else { 0 }
}

fn check_name(got: &str, expect: &str) -> i32 {
    streq(got, expect)
}

fn f() -> i32 {
    if check_name("f", "f") == 0 {
        -1
    } else {
        0
    }
}

fn g() -> i32 {
    if check_name("g", "g") == 0 {
        -2
    } else {
        0
    }
}

fn main() {
    if check_name("main", "main") == 0 {
        std::process::exit(1);
    }
    let result = f() + g();
    std::process::exit(result);
}