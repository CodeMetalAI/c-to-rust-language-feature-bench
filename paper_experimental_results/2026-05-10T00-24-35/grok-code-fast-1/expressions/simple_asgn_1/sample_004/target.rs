fn f() -> i32 {
    -1
}

fn main() {
    let c: i8 = f() as i8;
    let eq: i32 = if c as i32 == -1 { 1 } else { 0 };
    let char_is_signed: i32 = if (-1i8) < 0 { 1 } else { 0 };
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