fn fsize3(n: usize) -> usize {
    let mut b = vec![0; n + 3];
    b.capacity()
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    std::process::exit(0);
}