fn fsize3(n: usize) -> usize {
    let b = [0u8; 13]; // Array with size n + 3
    b.len()
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }
}