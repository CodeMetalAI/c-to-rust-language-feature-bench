fn fsize3(n: usize) -> usize {
    let b = [0u8; 100]; // Create a fixed-size array with a large enough capacity
    let size = n + 3;
    if size > b.len() {
        panic!("Array size exceeds maximum capacity");
    }
    size
}

fn main() {
    let size = fsize3(10);

    if size != 13 {
        std::process::exit(1);
    }

    std::process::exit(0);
}