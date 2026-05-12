fn main() {
    extern "C" {
        fn _Exit(status: i32) -> !;
    }

    static mut G: i32 = 0;

    fn die_if(x: i32) {
        unsafe {
            _Exit(if x == 7 && G == 9 { 0 } else { 2 })
        }
    }

    let p = die_if;
    unsafe {
        G = 9;
    }
    p(7);
}