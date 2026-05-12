fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;

    // In Rust, we need to explicitly assign and compare
    let val = f();
    c = val as i8;
    let eq = (c as i32) == -1;

    // Check if char (i8) is signed - in Rust, i8 is always signed
    let char_is_signed = true; // i8 is always signed in Rust

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