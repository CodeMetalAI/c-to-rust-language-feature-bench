fn main() {
    let mut dp = Box::new(3.14);
    if dp!= &3.14 {
        drop(dp);
        return 2;
    }
    drop(dp);
    return 0;
}