use std::process;

fn die_if(x: i32) -> ! {
    let exit_code = if x == 7 && crate::G == 9 { 0 } else { 2 };
    process::exit(exit_code);
}

static mut G: i32 = 0;

fn main() {
    let p: fn(i32) -> ! = die_if;
    unsafe {
        G = 9;
    }
    p(7);
}