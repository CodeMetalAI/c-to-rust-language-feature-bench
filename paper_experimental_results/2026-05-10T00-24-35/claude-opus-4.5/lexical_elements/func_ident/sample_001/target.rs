fn check_name(got: &str, expect: &str) -> bool {
    got == expect
}

fn f() -> i32 {
    fn current_fn() -> &'static str {
        "f"
    }
    if !check_name(current_fn(), "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    fn current_fn() -> &'static str {
        "g"
    }
    if !check_name(current_fn(), "g") {
        return -2;
    }
    0
}

fn main() {
    fn current_fn() -> &'static str {
        "main"
    }
    if !check_name(current_fn(), "main") {
        std::process::exit(1);
    }
    std::process::exit(f() + g());
}