fn main() {
    let mut a = [[0; 3]; 4];

    let p = &mut a[1];

    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p_index = 1; // Since p points to a[1], its index is 1
    if p_index != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}