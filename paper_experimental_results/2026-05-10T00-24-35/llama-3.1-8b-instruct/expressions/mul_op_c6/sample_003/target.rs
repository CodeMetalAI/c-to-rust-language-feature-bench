fn main() {
    let tests = vec![
        (17, 5, 3, -2),
        (-17, 5, -3, 1),
        (17, -5, -3, 14),
        (-17, -5, 3, -18),
    ];

    for (a, b, q, r) in tests {
        let mut q_result = a / b;
        let mut r_result = a % b;
        if q_result != q {
            println!("Test failed: q != {}", q);
            return;
        }
        if (q_result * b + r_result) != a {
            println!("Test failed: q * b + r != {}", a);
            return;
        }
    }

    println!("All tests passed");
}