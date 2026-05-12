fn main() -> i32 {
    let mut acc = 0;
    for _ in 0..5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
    }
    acc = 15;
    20 - acc
}