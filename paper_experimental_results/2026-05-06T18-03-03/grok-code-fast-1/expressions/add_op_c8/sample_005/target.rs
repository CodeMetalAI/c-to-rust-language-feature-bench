fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    let idx0: usize = 0;
    let idx2: usize = idx0 + 2;
    let idx4: usize = idx2 + 2;
    let idx_last: usize = 4;
    let idx_end: usize = idx_last + 1;

    if a[idx2] != 30 {
        std::process::exit(1);
    }
    if a[idx4] != 50 {
        std::process::exit(2);
    }

    let idx_back: usize = idx4 - 3;
    if a[idx_back] != 20 {
        std::process::exit(3);
    }

    let idx_from_end: usize = idx_end - 1;
    if idx_from_end != idx_last {
        std::process::exit(4);
    }
    if a[idx_from_end] != 50 {
        std::process::exit(5);
    }

    if (idx0 + 5) != idx_end {
        std::process::exit(6);
    }

    std::process::exit(0);
}