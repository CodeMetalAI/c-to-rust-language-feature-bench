use std::os::raw::c_char;
use std::process::exit;

fn f() -> i32 {
    -1
}

fn main() {
    let c: c_char = f() as c_char;
    let eq: i32 = (c as i32 == -1) as i32;

    let char_is_signed: bool = ((-1i8 as c_char) as i32) < 0;

    let exit_code = if char_is_signed {
        if eq != 1 { 1 } else { 0 }
    } else {
        if eq != 0 { 2 } else { 0 }
    };

    exit(exit_code);
}