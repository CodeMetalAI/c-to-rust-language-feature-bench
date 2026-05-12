fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let p0 = 0usize;
    let p2 = p0 + 2;
    let p4 = p2 + 2;
    let p_last = 4usize;
    let p_end = p_last + 1;
    if a[p2] != 30 {
        std::process::exit(1);
    }
    if a[p4] != 50 {
        std::process::exit(2);
    }
    let p_back = p4 - 3;
    if a[p_back] != 20 {
        std::process::exit(3);
    }
    let p_from_end = p_end - 1;
    if p_from_end != p_last {
        std::process::exit(4);
    }
    if a[p_from_end] != 50 {
        std::process::exit(5);
    }
    if (p0 + 5) != p_end {
        std::process::exit(6);
    }
    std::process::exit(0);
}