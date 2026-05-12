fn fsize3(n: i32) -> usize {
    let b: [u8; n as usize + 3] = [0; n as usize + 3];
    b.len()
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    std::process::exit(0);
}