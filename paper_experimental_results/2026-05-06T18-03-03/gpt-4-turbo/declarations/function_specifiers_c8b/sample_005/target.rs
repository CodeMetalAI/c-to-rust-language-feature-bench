use std::process::exit;

static mut G: i32 = 0;

fn die_if(x: i32) -> ! {
    unsafe {
        exit(if x == 7 && G == 9 { 0 } else { 2 });
    }
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    unsafe {
        G = 9;
    }
    p(7);
}