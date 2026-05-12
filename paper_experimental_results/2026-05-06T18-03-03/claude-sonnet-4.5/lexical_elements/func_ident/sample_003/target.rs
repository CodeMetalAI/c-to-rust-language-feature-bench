fn streq(a: &str, b: &str) -> bool {
    a == b
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
    let exit_code = if !check_name("main", "main") {
        1
    } else {
        f() + g()
    };
    std::process::exit(exit_code);
}