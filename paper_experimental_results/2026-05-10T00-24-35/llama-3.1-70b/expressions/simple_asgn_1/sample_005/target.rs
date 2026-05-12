fn f() -> i32 { -1 }

fn main() {
    let eq = (f() as i8) == -1;

    let char_is_signed = true;

    if char_is_signed {
        if eq != true {
            std::process::exit(1);
        }
    } else {
        if eq != false {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}