fn fsize3(n: usize) -> usize {
    let b = vec![0u8; n + 3];
    b.len()
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }
}