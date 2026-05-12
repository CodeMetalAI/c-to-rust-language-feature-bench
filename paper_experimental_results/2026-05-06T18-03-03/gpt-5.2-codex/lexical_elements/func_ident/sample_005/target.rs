fn streq(a: &str, b: &str) -> i32 {
    if a == b { 1 } else { 0 }
}

fn check_name(got: &str, expect: &str) -> i32 {
    streq(got, expect)
}

fn f() -> i32 {
    let func = "f";
    if check_name(func, "f") == 0 {
        return -1;
    }
    0
}

fn g() -> i32 {
    let func = "g";
    if check_name(func, "g") == 0 {
        return -2;
    }
    0
}

fn main() -> i32 {
    let func = "main";
    if check_name(func, "main") == 0 {
        return 1;
    }
    f() + g()
}