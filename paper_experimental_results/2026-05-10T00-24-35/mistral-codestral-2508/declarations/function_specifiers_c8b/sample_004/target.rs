fn die_if(x: i32) -> ! {
    if x == 7 && unsafe { G } == 9 {
        std::process::exit(0);
    } else {
        std::process::exit(2);
    }
}

static mut G: i32 = 0;

fn main() {
    let p: fn(i32) -> ! = die_if;
    unsafe {
        G = 9;
    }
    p(7);
}