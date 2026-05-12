fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(a, b)| a == b) && a.is_empty()
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if!check_name(__func__, "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if!check_name(__func__, "g") {
        return -2;
    }
    0
}

fn main() {
    if!check_name(__func__, "main") {
        std::process::exit(1);
    }
    f() + g()
}