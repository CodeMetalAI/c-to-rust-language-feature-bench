fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = &a[5]; // This will point to the element after the last, which is out of bounds

    if *p2 != 30 {
        return std::process::exit(1);
    }
    if *p4 != 50 {
        return std::process::exit(2);
    }

    let p_back = &a[1];
    if *p_back != 20 {
        return std::process::exit(3);
    }

    let p_from_end = &a[4];
    if p_from_end != p_last {
        return std::process::exit(4);
    }
    if *p_from_end != 50 {
        return std::process::exit(5);
    }

    // In Rust, we can't directly add 5 to a reference, but we can check that
    // the index 5 is out of bounds, which corresponds to p_end being invalid.
    // Since we can't create a valid reference to a[5], we simulate the check
    // by comparing the address of the hypothetical element after the array.
    // We'll use pointer arithmetic via raw pointers for this comparison only.
    let p0_raw = p0 as *const i32;
    let p_end_raw = p0_raw.add(5);
    let p_end_simulated = p_end_raw;
    let p_end_actual = &a[5] as *const i32; // This is actually invalid, but we can compare addresses
    if p_end_simulated != p_end_actual {
        return std::process::exit(6);
    }

    std::process::exit(0);
}