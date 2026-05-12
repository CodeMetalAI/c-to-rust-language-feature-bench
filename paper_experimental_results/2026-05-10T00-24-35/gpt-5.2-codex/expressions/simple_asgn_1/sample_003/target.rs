use std::os::raw::c_char;
use std::process::exit;

fn f() -> i32 {
    -1
}

fn main() {
    let mut c: c_char;

    let eq = {
        let temp = f();
        c = temp as c_char;
        (c as i32) == -1
    };

    let char_is_signed = ((-1i32 as c_char) as i32) < 0;

    if char_is_signed {
        if eq != true {
            exit(1);
        }
    } else {
        if eq != false {
            exit(2);
        }
    }

    exit(0);
}