fn die_if(x: i32) -> ! {
    static mut G: i32 = 0;
    unsafe {
        if x == 7 && G == 9 {
            std::process::exit(0);
        } else {
            std::process::exit(2);
        }
    }
}

static mut G: i32 = 0;

fn main() {
    let p: fn(i32) -> ! = |x| {
        unsafe {
            if x == 7 && G == 9 {
                std::process::exit(0);
            } else {
                std::process::exit(2);
            }
        }
    };
    unsafe {
        G = 9;
    }
    p(7);
}