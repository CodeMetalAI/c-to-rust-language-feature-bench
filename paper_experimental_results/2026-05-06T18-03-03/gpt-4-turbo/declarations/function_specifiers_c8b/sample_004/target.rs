use std::process::exit;

static mut G: i32 = 0;

fn die_if(x: i32) -> ! {
    unsafe {
        exit(if x == 7 && G == 9 { 0 } else { 2 });
    }
}

fn main() {
    unsafe {
        G = 9;
    }
    let p = die_if;
    p(7);
}