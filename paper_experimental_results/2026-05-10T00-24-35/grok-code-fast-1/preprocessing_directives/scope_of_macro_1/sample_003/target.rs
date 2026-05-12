fn main() {
    const ARRAY_SIZE: usize = 100;
    let a: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut tabs: i32 = 0;
    if a.len() != 100 {
        std::process::exit(1);
    }
    tabs = 7;
    if tabs != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}