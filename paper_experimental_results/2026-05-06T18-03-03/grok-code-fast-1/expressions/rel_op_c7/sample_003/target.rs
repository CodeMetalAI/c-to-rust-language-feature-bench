fn main() {
    let arr = [42, 0];
    let p = &arr[0];

    if p != &arr[0] {
        std::process::exit(1);
    }

    let p_end1 = &arr[1];
    let p_end2 = &arr[1];

    if p_end1 != p_end2 {
        std::process::exit(2);
    }

    if p_end1 == p {
        std::process::exit(3);
    }

    std::process::exit(0);
}