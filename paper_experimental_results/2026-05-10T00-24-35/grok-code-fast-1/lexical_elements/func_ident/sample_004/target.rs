fn streq(a: &str, b: &str) -> i32 {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut i = 0;
    loop {
        if a_bytes.get(i) != b_bytes.get(i) {
            return 0;
        }
        if let Some(&byte) = a_bytes.get(i) {
            if byte == 0 {
                return 1;
            }
        } else {
            // End of a, check if b also ended
            return if b_bytes.get(i).is_none() { 1 } else { 0 };
        }
        i += 1;
    }
}

fn check_name(got: &str, expect: &str) -> i32 {
    streq(got, expect)
}

fn f() -> i32 {
    if check_name("f", "f") != 1 {
        -1
    } else {
        0
    }
}

fn g() -> i32 {
    if check_name("g", "g") != 1 {
        -2
    } else {
        0
    }
}

fn main() {
    use std::process;
    let exit_code = if check_name("main", "main") != 1 {
        1
    } else {
        f() + g()
    };
    process::exit(exit_code);
}