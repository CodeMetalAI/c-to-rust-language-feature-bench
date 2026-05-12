fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;

    c = f() as i8;
    let eq = if (c as i32) == -1 { 1 } else { 0 };

    // In Rust, i8 is always signed, so char_is_signed is always true
    // But to match the C behavior exactly, we check if (i8)-1 < 0
    let char_is_signed = if (-1i8) < 0 { 1 } else { 0 };

    if char_is_signed != 0 {
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