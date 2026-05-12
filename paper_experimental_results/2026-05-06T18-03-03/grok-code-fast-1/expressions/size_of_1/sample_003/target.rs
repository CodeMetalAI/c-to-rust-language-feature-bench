fn main() -> i32 {
    let mut dp = Box::new(0.0);
    *dp = 3.14;
    if *dp != 3.14 {
        return 2;
    }
    0
}