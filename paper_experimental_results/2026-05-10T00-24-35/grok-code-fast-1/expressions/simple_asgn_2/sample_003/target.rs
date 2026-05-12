use std::process;

fn main() {
    let mut c: i8 = 0;
    let i: i32 = 100;
    let mut l: i64 = 0;

    c = i as i8;
    l = c as i64;

    if c != (i as i8) {
        process::exit(1);
    }
    if l != (c as i64) {
        process::exit(2);
    }
    if l != ((i as i8) as i64) {
        process::exit(3);
    }

    process::exit(0);
}