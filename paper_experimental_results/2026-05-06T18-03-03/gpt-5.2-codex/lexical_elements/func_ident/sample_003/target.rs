fn streq(a: &str, b: &str) -> i32 {
    if a == b { 1 } else { 0 }
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
    let mut exit_code = 0;
    if check_name("main", "main") == 0 {
        exit_code = 1;
    } else {
        exit_code = f() + g();
    }
    std::process::exit(exit_code);
}