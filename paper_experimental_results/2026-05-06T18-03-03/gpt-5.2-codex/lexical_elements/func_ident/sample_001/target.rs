use std::process;

fn streq(a: &str, b: &str) -> i32 {
    if a == b { 1 } else { 0 }
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
    let code = if check_name("main", "main") == 0 {
        1
    } else {
        f() + g()
    };
    process::exit(code);
}