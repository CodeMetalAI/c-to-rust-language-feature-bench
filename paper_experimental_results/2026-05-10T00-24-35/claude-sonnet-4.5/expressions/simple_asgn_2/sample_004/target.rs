fn main() {
    let i: i32 = 100;
    
    let c: i8 = i as i8;
    let l: i64 = c as i64;
    
    if c != (i as i8) {
        std::process::exit(1);
    }
    if l != (c as i64) {
        std::process::exit(2);
    }
    if l != ((i as i8) as i64) {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}