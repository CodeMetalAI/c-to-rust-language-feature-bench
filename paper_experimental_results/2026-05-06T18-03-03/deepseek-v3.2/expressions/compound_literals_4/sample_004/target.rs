fn main() {
    let p: &[f32] = &[1e0f, 1e1f, 1e2f, 1e3f, 1e4f, 1e5f, 1e6f];

    if p[0] != 1e0f {
        return std::process::exit(1);
    }
    if p[1] != 1e1f {
        return std::process::exit(2);
    }
    if p[2] != 1e2f {
        return std::process::exit(3);
    }
    if p[3] != 1e3f {
        return std::process::exit(4);
    }
    if p[4] != 1e4f {
        return std::process::exit(5);
    }
    if p[5] != 1e5f {
        return std::process::exit(6);
    }
    if p[6] != 1e6f {
        return std::process::exit(7);
    }

    std::process::exit(0);
}