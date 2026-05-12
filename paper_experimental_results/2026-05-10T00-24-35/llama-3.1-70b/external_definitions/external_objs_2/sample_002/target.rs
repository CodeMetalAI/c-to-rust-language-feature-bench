fn main() {
    static mut i: [i32; 1] = [0];

    if unsafe { i[0] }!= 0 {
        std::process::exit(1);
    }
    unsafe { i[0] = 7; }
    if unsafe { i[0] }!= 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}