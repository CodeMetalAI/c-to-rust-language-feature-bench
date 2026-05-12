fn main() {
    let x = 42;
    let p: usize = 0; // index representing &x

    if p != 0 {
        std::process::exit(1);
    }

    let p_end1 = p + 1;
    let p_end2 = 0 + 1; // represents &x + 1

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p {
        std::process::exit(3);
    }

    std::process::exit(0);
}