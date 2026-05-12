fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    let p0: usize = 0;
    let p2: usize = p0 + 2;
    let p4: usize = p2 + 2;
    let p_last: usize = 4;
    let p_end: usize = p_last + 1;

    if a[p2] != 30 {
        std::process::exit(1);
    }
    if a[p4] != 50 {
        std::process::exit(2);
    }

    let p_back: usize = p4 - 3;
    if a[p_back] != 20 {
        std::process::exit(3);
    }

    let p_from_end: usize = p_end - 1;
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