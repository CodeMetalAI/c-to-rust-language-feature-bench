fn fsize3(n: usize) -> usize {
    let b: [u8; n + 3] = [0; n + 3];
    return b.len();
}

fn main() {
    let size = fsize3(10);
    
    if size != 13 {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}