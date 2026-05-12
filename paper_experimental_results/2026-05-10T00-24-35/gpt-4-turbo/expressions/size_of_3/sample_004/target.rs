fn fsize3(n: usize) -> usize {
    n + 3
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }
}