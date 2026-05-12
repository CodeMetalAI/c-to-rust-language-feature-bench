fn main() {
    let mut c: i8; // Assuming `char` in C is similar to `i8` in Rust.

    let f = || -> i32 { -1 };

    c = f() as i8;
    let eq = (c == -1i8) as i32;

    let char_is_signed = (i8::MIN < 0);

    if char_is_signed {
        if eq != 1 {
            std::process::exit(1);
        }
    } else {
        if eq != 0 {
            std::process::exit(2);
        }
    }
}