fn main() {
    let w = [[1, 0, 0], 2];

    if w.len()!= 2 {
        std::process::exit(1);
    }

    if w[0].0[0]!= 1 {
        std::process::exit(2);
    }
    if w[0].0[1]!= 0 {
        std::process::exit(3);
    }
    if w[0].0[2]!= 0 {
        std::process::exit(4);
    }
    if w[0].1!= 0 {
        std::process::exit(5);
    }

    if w[1].0[0]!= 2 {
        std::process::exit(6);
    }
    if w[1].0[1]!= 0 {
        std::process::exit(7);
    }
    if w[1].0[2]!= 0 {
        std::process::exit(8);
    }
    if w[1].1!= 0 {
        std::process::exit(9);
    }

    std::process::exit(0);
}