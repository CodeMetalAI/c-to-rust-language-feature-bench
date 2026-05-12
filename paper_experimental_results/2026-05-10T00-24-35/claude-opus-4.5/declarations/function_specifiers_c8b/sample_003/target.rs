use std::process;

static mut G: i32 = 0;

fn die_if(x: i32) -> ! {
    unsafe {
        if x == 7 && G == 9 {
            process::exit(0);
        } else {
            process::exit(2);
        }
    }
}

fn main() {
    let p: fn(i32) -> ! = die_if;
    unsafe {
        G = 9;
    }
    p(7);
}