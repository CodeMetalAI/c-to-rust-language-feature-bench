fn main() {
    let mut c: i8;

    let f = || -1;
    c = f();

    let eq = (c == -1) as i32;

    let char_is_signed = (-1i8 < 0) as i32;

    if char_is_signed == 1 {
        if eq != 1 {
            std::process::exit(1);
        }
    } else {
        if eq != 0 {
            std::process::exit(2);
        }
    }
}