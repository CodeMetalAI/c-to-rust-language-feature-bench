fn main() {
    let p = [1e0f, 1e1f, 1e2f, 1e3f, 1e4f, 1e5f, 1e6f];

    if p[0] != 1e0f {
        return 1;
    }
    if p[1] != 1e1f {
        return 2;
    }
    if p[2] != 1e2f {
        return 3;
    }
    if p[3] != 1e3f {
        return 4;
    }
    if p[4] != 1e4f {
        return 5;
    }
    if p[5] != 1e5f {
        return 6;
    }
    if p[6] != 1e6f {
        return 7;
    }

    println!("0");
}