fn fsize3(n: i32) -> usize {
    let b = [0u8; 13]; // Fixed size array, n+3 where n=10
    std::mem::size_of_val(&b)
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }
}