use std::os::raw::c_char;
use std::process::exit;

fn f() -> i32 {
    -1
}

fn main() {
    let c: c_char = f() as c_char;

    let eq: i32 = if (c as i32) == -1 { 1 } else { 0 };

    let char_is_signed: bool = ((-1 as c_char) as i32) < 0;

    if char_is_signed {
        if eq != 1 {
            exit(1);
        }
    } else {
        if eq != 0 {
            exit(2);
        }
    }

    exit(0);
}