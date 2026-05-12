fn f() -> i32 {
    -1
}

fn main() {
    let c: i32 = f();

    let char_is_signed = (i32::MIN < 0);

    if char_is_signed {
        if c != -1 {
            std::process::exit(1);
        }
    } else {
        if c != 0 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}