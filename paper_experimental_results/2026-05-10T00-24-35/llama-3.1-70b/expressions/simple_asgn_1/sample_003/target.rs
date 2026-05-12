fn f() -> i32 {
    -1
}

fn main() {
    let c: i8 = f();

    let eq = c == -1;

    let char_is_signed = (-1 as i8) < 0;

    if char_is_signed {
        if eq!= true {
            std::process::exit(1);
        }
    } else {
        if eq!= false {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}