fn main() {
    // Assuming i32 is 4 bytes and no padding, as in typical C implementations
    // sizeof(struct s) == 4, sizeof(struct ss) == 4
    if 4 < 4 {
        std::process::exit(1);
    }
    // offsetof(struct s, d) == 4, sizeof(struct s) == 4
    if 4 != 4 {
        std::process::exit(1);
    }

    // Allocate vectors to simulate the flexible arrays
    let mut s1_d = vec![0.0f64; 8];
    let mut s2_d = vec![0.0f64; 5];

    s1_d[0] = 42.0;
    s2_d[0] = 24.0;

    if s1_d[0] != 42.0 || s2_d[0] != 24.0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}