use std::os::raw::c_char;

fn f() -> i32 {
    -1
}

fn main() {
    let mut c: c_char = 0;
    c = f() as c_char;

    let eq: i32 = if (c as i32) == -1 { 1 } else { 0 };
    let char_is_signed: bool = ((-1i32 as c_char) as i32) < 0;

    let exit_code = if char_is_signed {
        if eq != 1 { 1 } else { 0 }
    } else {
        if eq != 0 { 2 } else { 0 }
    };

    std::process::exit(exit_code);
}