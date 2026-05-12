fn main() {
    let mut i: Vec<i32> = vec![0];
    
    if i[0] != 0 {
        std::process::exit(1);
    }
    i[0] = 7;
    if i[0] != 7 {
        std::process::exit(2);
    }
    std::process::exit(0);
}