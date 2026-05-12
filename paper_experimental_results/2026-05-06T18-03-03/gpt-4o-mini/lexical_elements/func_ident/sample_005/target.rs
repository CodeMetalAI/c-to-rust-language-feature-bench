fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name(std::any::type_name::<fn()>(), "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if !check_name(std::any::type_name::<fn()>(), "g") {
        return -2;
    }
    0
}

fn main() -> i32 {
    if !check_name(std::any::type_name::<fn()>(), "main") {
        return 1;
    }
    f() + g()
}