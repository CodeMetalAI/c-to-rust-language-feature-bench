fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;
    let eq: bool;

    unsafe {
        // Use unsafe block to mimic the behavior of assigning to a potentially uninitialized variable
        // This is not idiomatic Rust, but it's necessary to match the original C/C++ behavior
        let mut temp_c: i8 = 0;
        temp_c = f() as i8;
        c = temp_c;
        eq = c == -1;
    }

    let char_is_signed = (-1 as i8) < 0;

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