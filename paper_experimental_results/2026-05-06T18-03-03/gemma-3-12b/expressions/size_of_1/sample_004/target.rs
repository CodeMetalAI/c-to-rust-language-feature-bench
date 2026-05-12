fn main() {
    let mut dp: Vec<f64> = Vec::with_capacity(1);

    if dp.len() == 0 {
        return 1;
    }

    dp.push(3.14);

    if dp[0] != 3.14 {
        return 2;
    }

    return 0;
}