fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    let p0_idx = 0;
    let p2_idx = p0_idx + 2;
    let p4_idx = p2_idx + 2;
    let p_last_idx = 4;
    let p_end_idx = p_last_idx + 1;

    if a[p2_idx] != 30 {
        std::process::exit(1);
    }
    if a[p4_idx] != 50 {
        std::process::exit(2);
    }

    let p_back_idx = p4_idx - 3;
    if a[p_back_idx] != 20 {
        std::process::exit(3);
    }

    let p_from_end_idx = p_end_idx - 1;
    if p_from_end_idx != p_last_idx {
        std::process::exit(4);
    }
    if a[p_from_end_idx] != 50 {
        std::process::exit(5);
    }

    if (p0_idx + 5) != p_end_idx {
        std::process::exit(6);
    }

    std::process::exit(0);
}