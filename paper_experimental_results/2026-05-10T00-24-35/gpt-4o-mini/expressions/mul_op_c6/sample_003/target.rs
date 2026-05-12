fn main() {
    let test_cases = [
        (17, 5),
        (-17, 5),
        (17, -5),
        (-17, -5),
    ];

    for (a, b) in &test_cases {
        let q = a / b;
        let r = a % b;

        if (*a >= 0 && *b > 0 && q != 3) || (*a < 0 && *b > 0 && q != -3) ||
           (*a >= 0 && *b < 0 && q != -3) || (*a < 0 && *b < 0 && q != 3) {
            std::process::exit(match (a, b) {
                (17, 5) => 1,
                (-17, 5) => 3,
                (17, -5) => 5,
                (-17, -5) => 7,
                _ => unreachable!(),
            });
        }

        if q * b + r != *a {
            std::process::exit(match (a, b) {
                (17, 5) => 2,
                (-17, 5) => 4,
                (17, -5) => 6,
                (-17, -5) => 8,
                _ => unreachable!(),
            });
        }
    }

    std::process::exit(0);
}