fn fsize3(n: i32) -> usize {
    (n + 3) as usize
}

fn main() {
    let size = fsize3(10);
    if size != 13 {
        std::process::exit(1);
    }
    std::process::exit(0);
}