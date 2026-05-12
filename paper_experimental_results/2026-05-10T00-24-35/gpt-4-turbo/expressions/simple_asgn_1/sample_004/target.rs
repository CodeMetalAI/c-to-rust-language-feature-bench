fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i8;

    c = f() as i8;
    let eq = (c == -1) as i32;

    let char_is_signed = (i8::MIN < 0) as i32;

    if char_is_signed != 0 {
        if eq != 1 {
            std::process::exit(1);
        }
    } else {
        if eq != 0 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}