fn fsize3(n: usize) -> usize {
    let b: Vec<u8> = vec![0; n + 3];
    b.len()
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    std::process::exit(0);
}