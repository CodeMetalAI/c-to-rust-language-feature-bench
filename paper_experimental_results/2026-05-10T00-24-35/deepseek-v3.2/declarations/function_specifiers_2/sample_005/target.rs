#![feature(never_type)]

use std::process;

fn stop() -> ! {
    let mut x = 0;
    loop {
        x += 1;
    }
}

fn f() -> ! {
    stop()
}

fn g(i: i32) {
    if i > "0".parse().unwrap() {
        stop();
    }
    process::exit(0);
}

fn main() {
    if true {
        f();
    }

    g(1);
}