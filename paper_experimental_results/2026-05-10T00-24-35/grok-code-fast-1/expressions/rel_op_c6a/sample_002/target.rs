use std::process;

fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 2;

    if !(((a < b) as i32) < c) {
        process::exit(1);
    }

    if !(((a < b) as i32) < c) {
        process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    if !(((a < b) as i32) < c) {
        process::exit(3);
    }

    process::exit(0);
}