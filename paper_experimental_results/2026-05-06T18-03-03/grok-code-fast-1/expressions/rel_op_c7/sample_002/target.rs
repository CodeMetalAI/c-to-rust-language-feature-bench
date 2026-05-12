fn main() {
    let v = vec![42];
    let p = 0;
    let x_addr = 0;

    if p != x_addr {
        std::process::exit(1);
    }

    let p_end1 = p + 1;
    let p_end2 = x_addr + 1;

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p {
        std::process::exit(3);
    }

    std::process::exit(0);
}