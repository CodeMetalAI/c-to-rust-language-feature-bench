fn fsize3(n: isize) -> usize {
    let b: [u8; 13] = [0; 13];
    std::mem::size_of_val(&b)
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }
}