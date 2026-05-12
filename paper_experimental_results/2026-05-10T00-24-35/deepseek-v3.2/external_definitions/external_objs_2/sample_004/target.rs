fn main() {
    static mut I: [i32; 1] = [0];
    
    if I[0] != 0 {
        return std::process::exit(1);
    }
    I[0] = 7;
    if I[0] != 7 {
        return std::process::exit(2);
    }
    std::process::exit(0);
}