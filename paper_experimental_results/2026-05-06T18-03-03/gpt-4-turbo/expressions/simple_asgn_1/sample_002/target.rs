fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i8 = 0;
    let eq = {
        c = f() as i8;
        c == -1
    };

    let char_is_signed = (-1_i8) < 0;

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