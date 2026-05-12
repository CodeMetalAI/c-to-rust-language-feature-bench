fn f() -> i32 {
    -1
}

fn main() {
    let c = f() as i8;
    let eq = if c as i32 == -1 { 1 } else { 0 };
    let char_is_signed = if (-1i8 as i32) < 0 { 1 } else { 0 };
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