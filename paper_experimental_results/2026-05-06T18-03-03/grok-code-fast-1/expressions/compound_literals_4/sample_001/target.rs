fn main() {
    let arr: [f32; 7] = [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6];
    let p = &arr;

    if p[0] != 1e0 {
        std::process::exit(1);
    }
    if p[1] != 1e1 {
        std::process::exit(2);
    }
    if p[2] != 1e2 {
        std::process::exit(3);
    }
    if p[3] != 1e3 {
        std::process::exit(4);
    }
    if p[4] != 1e4 {
        std::process::exit(5);
    }
    if p[5] != 1e5 {
        std::process::exit(6);
    }
    if p[6] != 1e6 {
        std::process::exit(7);
    }

    std::process::exit(0);
}