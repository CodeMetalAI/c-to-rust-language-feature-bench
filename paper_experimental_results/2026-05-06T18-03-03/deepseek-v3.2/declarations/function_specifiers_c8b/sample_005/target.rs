use std::process;

fn die_if(x: i32) -> ! {
    let g = unsafe { G };
    process::exit(if x == 7 && g == 9 { 0 } else { 2 });
}

static mut G: i32 = 0;

fn main() {
    let p: fn(i32) -> ! = die_if;
    unsafe { G = 9 };
    p(7);
}