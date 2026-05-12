fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(x, y)| x == y) && a.is_empty() ? true : false
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name("f", stringify!(f)) {
        -1
    } else {
        0
    }
}

fn g() -> i32 {
    if !check_name("g", stringify!(g)) {
        -2
    } else {
        0
    }
}

fn main() -> i32 {
    if !check_name("main", stringify!(main)) {
        1
    } else {
        f() + g()
    }
}