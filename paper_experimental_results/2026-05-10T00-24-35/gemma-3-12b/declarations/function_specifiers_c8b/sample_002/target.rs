fn main() -> i32 {
    static mut G: i32 = 0;
    unsafe {
        G = 9;
    }
    let p = |x: i32| {
        if x == 7 && unsafe { G == 9 } {
            std::process::exit(0);
        } else {
            std::process::exit(2);
        }
    };
    p(7);
    3
}