use std::os::raw::c_char;

fn f() -> i32 {
    -1
}

fn main() {
    let c: c_char = f() as c_char;
    let eq: i32 = if (c as i32) == -1 { 1 } else { 0 };

    let minus_one: c_char = (-1i8) as c_char;
    let zero: c_char = 0 as c_char;
    let char_is_signed = minus_one < zero;

    let exit_code = if char_is_signed {
        if eq != 1 { 1 } else { 0 }
    } else {
        if eq != 0 { 2 } else { 0 }
    };

    std::process::exit(exit_code);
}