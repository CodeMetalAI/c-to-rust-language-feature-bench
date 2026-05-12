use std::os::raw::c_char;

fn f() -> i32 {
    -1
}

fn main() {
    let mut c: c_char = 0;

    let eq: i32 = {
        c = f() as c_char;
        if (c as i32) == -1 { 1 } else { 0 }
    };

    let char_is_signed: i32 = if (-1 as c_char) < (0 as c_char) { 1 } else { 0 };

    let exit_code = if char_is_signed != 0 {
        if eq != 1 { 1 } else { 0 }
    } else {
        if eq != 0 { 2 } else { 0 }
    };

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}