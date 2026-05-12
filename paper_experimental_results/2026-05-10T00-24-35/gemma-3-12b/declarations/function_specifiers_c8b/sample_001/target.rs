fn main() -> i32 {
    static mut g: i32 = 0;

    fn die_if(x: i32) {
        unsafe {
            if x == 7 && *g == 9 {
                std::process::exit(0);
            } else {
                std::process::exit(2);
            }
        }
    }

    unsafe {
        g = 9;
    }
    die_if(7);
    3
}