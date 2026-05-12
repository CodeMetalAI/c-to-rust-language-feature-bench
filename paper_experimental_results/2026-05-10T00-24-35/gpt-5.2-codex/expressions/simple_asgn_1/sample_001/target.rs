use std::os::raw::c_char;

fn f() -> i32 {
    -1
}

fn main() {
    let c: c_char = f() as c_char;

    let eq: i32 = if (c as i32) == -1 { 1 } else { 0 };

    let char_is_signed: bool = (-1 as c_char) < (0 as c_char);

    let code: i32 = if char_is_signed {
        if eq != 1 { 1 } else { 0 }
    } else {
        if eq != 0 { 2 } else { 0 }
    };

    std::process::exit(code);
}