fn main() {
    fn die_if(x: i32) -> ! {
        std::process::exit(if x == 7 && G == 9 { 0 } else { 2 });
    }

    static mut G: i32 = 0;

    let p: fn(i32) -> ! = die_if;
    unsafe {
        G = 9;
    }
    p(7);
}