use std::process;

static mut G: i32 = 0;

fn die_if(x: i32) -> ! {
    let code = if x == 7 && unsafe { G } == 9 { 0 } else { 2 };
    process::exit(code);
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    unsafe { G = 9; }
    p(7);
}