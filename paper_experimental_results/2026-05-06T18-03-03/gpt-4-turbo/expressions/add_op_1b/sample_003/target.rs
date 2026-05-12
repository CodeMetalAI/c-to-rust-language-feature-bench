fn main() {
    let mut a = [[0; 3]; 4];

    {
        let p = &mut a[1..];
        p[0][2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // Rust does not have direct pointer arithmetic, but we can simulate the check
    let p = &a[1] as *const _;
    let base = &a[0] as *const _;
    if unsafe { p.offset_from(base) } != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}