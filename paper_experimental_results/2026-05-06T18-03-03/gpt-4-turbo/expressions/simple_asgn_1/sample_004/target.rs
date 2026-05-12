fn main() {
    let f = || -> i32 { -1 };

    let c = f();
    let eq = (c == -1);

    let char_is_signed = (i8::MIN < 0);

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