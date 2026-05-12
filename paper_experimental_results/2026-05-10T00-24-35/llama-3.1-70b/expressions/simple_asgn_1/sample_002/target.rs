fn f() -> i32 { -1 }

fn main() {
    let c = f() as i8;
    let eq = (c == -1) as i32;

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