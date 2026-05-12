fn f() -> i32 {
    -1
}

fn main() {
    let mut c: i8 = 0;
    c = f() as i8;
    let eq: i32 = (c as i32 == -1) as i32;
    let char_is_signed: i32 = ((-1i8 as i32) < 0) as i32;
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