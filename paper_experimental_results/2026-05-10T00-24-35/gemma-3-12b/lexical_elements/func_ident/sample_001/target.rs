fn streq(a: &str, b: &str) -> bool {
    let mut i = 0;
    loop {
        if i >= a.len() || i >= b.len() {
            if i >= a.len() && i >= b.len() {
                return true;
            } else {
                return false;
            }
        }
        if a.as_bytes()[i] != b.as_bytes()[i] {
            return false;
        }
        if a.as_bytes()[i] == 0 {
            return true;
        }
        i += 1;
    }
}

fn check_name(got: &str, expect: &str) -> bool {
    streq(got, expect)
}

fn f() -> i32 {
    if !check_name(__func__, "f") {
        return -1;
    }
    0
}

fn g() -> i32 {
    if !check_name(__func__, "g") {
        return -2;
    }
    0
}

fn main() -> i32 {
    if !check_name(__func__, "main") {
        return 1;
    }
    f() + g()
}

#[macro_export]
macro_rules! __func {
    () => {
        std::module::Module::new("").unwrap().name()
    };
}