fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(c1, c2)| c1 == c2) && a.is_empty() || b.is_empty()
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name(__func__, "f") {
        -1
    } else {
        0
    }
}

fn g() -> i32 {
    if !check_name(__func__, "g") {
        -2
    } else {
        0
    }
}

fn main() -> i32 {
    if !check_name(__func__, "main") {
        1
    } else {
        f() + g()
    }
}