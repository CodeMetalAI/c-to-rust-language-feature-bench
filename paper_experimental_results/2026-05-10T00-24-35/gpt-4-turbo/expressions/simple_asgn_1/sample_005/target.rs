fn main() {
    let mut c: i8;

    let f = || -> i32 { -1 };

    c = f() as i8;
    let eq = c == -1;

    let char_is_signed = (-1i8) < 0;

    if char_is_signed {
        if !eq {
            std::process::exit(1);
        }
    } else {
        if eq {
            std::process::exit(2);
        }
    }
}