fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i8;
    let eq = {
        c = f() as i8;
        if (c as i32) == -1 { 1 } else { 0 }
    };
    let char_is_signed = if ((i8::from(-1)) as i32) < 0 { 1 } else { 0 };
    if char_is_signed == 1 {
        if eq != 1 {
            std::process::exit(1);
        }
    } else {
        if eq != 0 {
            std::process::exit(2);
        }
    }
    std::process::exit(0);
}