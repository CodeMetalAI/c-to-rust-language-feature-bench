fn main() {
    let p: [&'static f64; 7] = [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6];

    if p[0] != 1e0 {
        return 1;
    }
    if p[1] != 1e1 {
        return 2;
    }
    if p[2] != 1e2 {
        return 3;
    }
    if p[3] != 1e3 {
        return 4;
    }
    if p[4] != 1e4 {
        return 5;
    }
    if p[5] != 1e5 {
        return 6;
    }
    if p[6] != 1e6 {
        return 7;
    }

    std::process::exit(0);
}