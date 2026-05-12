fn f() -> i32 {
    -1
}

fn main() {
    let c: char;

    let eq = c == f();

    let char_is_signed = -1 < 0;

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