fn main() {
    let w = [
        { let mut a = [0; 3]; a[0] = 1; a },
        { let mut a = [0; 3]; a[0] = 2; a },
    ];

    if w.len() != 2 {
        std::process::exit(1);
    }

    if w[0][0] != 1 {
        std::process::exit(2);
    }
    if w[0][1] != 0 {
        std::process::exit(3);
    }
    if w[0][2] != 0 {
        std::process::exit(4);
    }

    if w[1][0] != 2 {
        std::process::exit(6);
    }
    if w[1][1] != 0 {
        std::process::exit(7);
    }
    if w[1][2] != 0 {
        std::process::exit(8);
    }
}