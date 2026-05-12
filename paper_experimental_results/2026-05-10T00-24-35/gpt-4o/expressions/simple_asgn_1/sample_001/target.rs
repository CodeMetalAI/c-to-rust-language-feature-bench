fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;  // Use i8 to represent signed char

    let eq = {
        c = f() as i8;
        c == -1
    };

    let char_is_signed = (-1i8 < 0);

    let exit_code = if char_is_signed {
        if eq != true {
            1
        } else {
            0
        }
    } else {
        if eq != false {
            2
        } else {
            0
        }
    };

    std::process::exit(exit_code);
}