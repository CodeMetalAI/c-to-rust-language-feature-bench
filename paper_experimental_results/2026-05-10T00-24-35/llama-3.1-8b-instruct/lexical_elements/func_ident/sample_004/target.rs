fn streq(a: &str, b: &str) -> bool {
    let mut i = 0;
    while a.chars().nth(i).unwrap_or('\0') == b.chars().nth(i).unwrap_or('\0') {
        if a.chars().nth(i).unwrap_or('\0') == '\0' {
            return true;
        }
        i += 1;
    }
    false
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name(__func_name(), "f") {
        -1
    } else {
        0
    }
}

fn g() -> i32 {
    if !check_name(__func_name(), "g") {
        -2
    } else {
        0
    }
}

fn main() {
    if !check_name(__func_name(), "main") {
        1
    } else {
        f() + g()
    }
}