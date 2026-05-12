fn f() -> i32 {
    -1
}

fn main() {
    let c: i8;

    let eq = {
        c = f() as i8;
        c == -1
    };

    let char_is_signed = (-1i8 as i32) < 0;

    if char_is_signed {
        if !eq {
            std::process::exit(1);
        }
    } else {
        if eq {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}