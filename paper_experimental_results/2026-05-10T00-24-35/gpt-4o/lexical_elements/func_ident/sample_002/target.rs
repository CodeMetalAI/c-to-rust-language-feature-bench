fn streq(a: &str, b: &str) -> bool {
    let mut chars_a = a.chars();
    let mut chars_b = b.chars();

    loop {
        match (chars_a.next(), chars_b.next()) {
            (Some(ac), Some(bc)) if ac == bc => continue,
            (None, None) => return true,
            _ => return false,
        }
    }
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

fn main() -> i32 {
    if !check_name("main", "main") {
        return 1;
    }
    f() + g()
}