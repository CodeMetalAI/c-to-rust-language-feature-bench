fn fsize3(n: i32) -> usize {
    (n as usize) + 3
}

fn main() -> i32 {
    let size = fsize3(10);
    if size != 13 {
        return 1;
    }
    0
}