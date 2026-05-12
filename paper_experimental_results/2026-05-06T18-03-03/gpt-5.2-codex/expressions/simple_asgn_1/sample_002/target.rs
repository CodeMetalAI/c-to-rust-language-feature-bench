use std::os::raw::c_char;

fn f() -> i32 {
    -1
}

fn main() {
    let mut c: c_char = 0;

    let eq = ({
        c = f() as c_char;
        (c as i32) == -1
    }) as i32;

    let char_is_signed = ((-1i32 as c_char) as i32) < 0;

    let mut ret = 0;
    if char_is_signed {
        if eq != 1 {
            ret = 1;
        }
    } else {
        if eq != 0 {
            ret = 2;
        }
    }

    std::process::exit(ret);
}