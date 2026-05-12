fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;

    // In Rust, assignment is not an expression, so we must split it
    c = f() as i8;
    let eq = (c as i32 == -1);

    let char_is_signed = (i8::from(-1i32) < 0);

    if char_is_signed {
        if !eq {
            std::process::exit(1);
        }
    } else {
        if eq {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}