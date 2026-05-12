fn main() {
    let p: &[f32] = &[1.0, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0];

    if p[0] != 1.0 {
        std::process::exit(1);
    }
    if p[1] != 10.0 {
        std::process::exit(2);
    }
    if p[2] != 100.0 {
        std::process::exit(3);
    }
    if p[3] != 1000.0 {
        std::process::exit(4);
    }
    if p[4] != 10000.0 {
        std::process::exit(5);
    }
    if p[5] != 100000.0 {
        std::process::exit(6);
    }
    if p[6] != 1000000.0 {
        std::process::exit(7);
    }

    std::process::exit(0);
}