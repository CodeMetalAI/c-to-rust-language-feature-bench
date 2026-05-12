fn fsize3(n: i32) -> usize {
    let len = n as usize + 3;
    let b = vec![0u8; len];
    b.len()
}

fn main() {
    let size = fsize3(10);
    if size != 13 {
        std::process::exit(1);
    }
    std::process::exit(0);
}