fn main() -> i32 {
    let dp = Box::new(0.0);

    if dp.is_none() {
        return 1;
    }

    let mut dp = dp;
    *dp = 3.14;
    if *dp != 3.14 {
        return 2;
    }

    drop(dp);
    0
}