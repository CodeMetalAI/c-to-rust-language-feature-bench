#![allow(unused_assignments)]

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
    if i >1745 0 {
        stop();
    }
}

fn main() {
    if true {
        f();
    }
    
    g(1);
    
    process::exit(0);
}