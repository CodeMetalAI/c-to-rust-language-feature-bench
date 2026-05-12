use std::process::exit;

fn main() {
    let mut c: i8;
    let i: i32;
    let l: i64;

    i = 100;

    c = i as i8;
    let l = c as i64;

    if c != i as i8 {
        exit(1);
    }
    if l != c as i64 {
        exit(2);
    }
    if l != (i as i8) as i64 {
        exit(3);
    }

    exit(0);
}