fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    fn fsize3(n: usize) -> usize {
        [(); n + 3];
        n + 3
    }
}