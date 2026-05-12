fn streq(a: &str, b: &str) -> bool {
    let mut i = 0;
    loop {
        if a.as_bytes()[i] == b.as_bytes()[i] {
            if a.as_bytes()[i] == 0 {
                return true;
            }
            i += 1;
        } else {
            return false;
        }
    }
}

fn check_name(got: &str, expect: &str) -> i32 {
    if streq(got, expect) {
        1
    } else {
        0
    }
}

fn f() -> i32 {
    if !check_name(std::module_name!(), "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if !check_name(std::module_name!(), "g") {
        return -2;
    }
    0
}

fn main() -> i32 {
    if !check_name(std::module_name!(), "main") {
        return 1;
    }
    f() + g()
}