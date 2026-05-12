fn streq(a: &str, b: &str) -> i32 {
    if a == b { 1 } else { 0 }
}

fn check_name(got: &str, expect: &str) -> i32 {
    streq(got, expect)
}

fn f() -> i32 {
    if check_name("f", "f") != 1 {
        -1
    } else {
        0
    }
}

fn g() -> i32 {
    if check_name("g", "g") != 1 {
        -2
    } else {
        0
    }
}

fn main() {
    if check_name("main", "main") != 1 {
        std::process::exit(1);
    }
    let res = f() + g();
    std::process::exit(res);
}