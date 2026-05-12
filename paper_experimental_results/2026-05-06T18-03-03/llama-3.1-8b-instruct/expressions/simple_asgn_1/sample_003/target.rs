fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i32 = f();

    let eq = c == -1;

    let char_is_signed = -1 < 0;

    if char_is_signed {
        if eq != 1 {
            std::process::exit(1);
        }
    } else {
        if eq != 0 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}