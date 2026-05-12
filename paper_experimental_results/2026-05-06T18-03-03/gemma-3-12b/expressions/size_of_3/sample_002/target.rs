fn main() {
    let size = fsize3(10);

    if size != 13 {
        return 1;
    }

    return 0;
}

fn fsize3(n: i32) -> usize {
    let b: [u8; (n + 3) as usize] = [0; (n + 3) as usize];
    b.len()
}