use std::process::exit;

fn f(init: i32) -> i32 {
    let x = init;
    let p = &x;

    let arr = [*p, 0];

    if arr[0] != init {
        return 1;
    }
    if arr[1] != 0 {
        return 2;
    }

    0
}

fn main() {
    if f(7) != 0 {
        exit(10);
    }
    if f(-3) != 0 {
        exit(11);
    }
    exit(0);
}