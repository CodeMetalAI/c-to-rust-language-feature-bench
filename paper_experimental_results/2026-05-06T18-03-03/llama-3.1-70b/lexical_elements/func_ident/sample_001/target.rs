fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if!check_name("f", "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if!check_name("g", "g") {
        return -2;
    }
    0
}

fn main() {
    if!check_name("main", "main") {
        std::process::exit(1);
    }
    std::process::exit(f() + g());
}