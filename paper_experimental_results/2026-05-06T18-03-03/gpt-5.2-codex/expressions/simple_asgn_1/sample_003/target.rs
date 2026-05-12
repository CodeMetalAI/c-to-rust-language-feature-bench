use std::os::raw::c_char;
use std::process;

fn f() -> i32 {
    -1
}

fn main() {
    let mut c: c_char = 0;

    let eq = {
        c = f() as c_char;
        if (c as i32) == -1 { 1 } else { 0 }
    };

    let char_is_signed = ((-1i8 as c_char) as i32) < 0;

    if char_is_signed {
        if eq != 1 {
            process::exit(1);
        }
    } else {
        if eq != 0 {
            process::exit(2);
        }
    }

    process::exit(0);
}