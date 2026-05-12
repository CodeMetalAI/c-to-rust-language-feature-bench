// function_specifiers_c8b.rs

use std::process::exit;

fn die_if(x: i32) -> ! {
    if x == 7 && g == 9 {
        exit(0);
    } else {
        exit(2);
    }
}

static mut G: i32 = 9;

fn main() -> ! {
    let p: fn(i32) -> ! = die_if;
    unsafe { G = 9 };
    p(7);
    std::process::exit(3);
}