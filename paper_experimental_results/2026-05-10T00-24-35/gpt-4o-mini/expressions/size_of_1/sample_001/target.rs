fn alloc(n: usize) -> Option<Box<[f64]>> {
    if n == 0 {
        None
    } else {
        Some(vec![0.0; n].into_boxed_slice())
    }
}

fn main() -> i32 {
    let dp = alloc(1).unwrap_or_else(|| return 1);

    dp[0] = 3.14;
    if dp[0] != 3.14 {
        return 2;
    }

    // No need to explicitly free in Rust, it will be freed automatically
    0
}