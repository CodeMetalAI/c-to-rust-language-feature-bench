fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i8 = 0;

    let eq = (c = f()) == -1;

    let char_is_signed = (c as i32) as i32 < 0;

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