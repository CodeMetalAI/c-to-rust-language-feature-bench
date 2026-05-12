fn main() {
    let x: i32 = 42;
    let p: usize = 0;

    if p != 0 {
        std::process::exit(1);
    }

    let p_end1: usize = p + 1;
    let p_end2: usize = 1;

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p {
        std::process::exit(3);
    }

    std::process::exit(0);
}