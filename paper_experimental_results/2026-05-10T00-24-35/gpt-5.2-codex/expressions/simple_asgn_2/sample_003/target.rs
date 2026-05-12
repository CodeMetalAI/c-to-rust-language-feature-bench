use std::process::exit;

fn main() {
    let i: i32 = 100;
    let c: i8 = i as i8;
    let l: i64 = c as i64;

    let code = if c != i as i8 {
        1
    } else if l != c as i64 {
        2
    } else if l != (i as i8) as i64 {
        3
    } else {
        0
    };

    exit(code);
}