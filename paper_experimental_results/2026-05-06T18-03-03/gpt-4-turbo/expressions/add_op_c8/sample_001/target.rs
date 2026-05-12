fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = &a[5]; // This is one past the last element, similar to C++'s end() iterator.

    if *p2 != 30 {
        std::process::exit(1);
    }
    if *p4 != 50 {
        std::process::exit(2);
    }

    let p_back = &a[1]; // p4 - 3 points to a[1]
    if *p_back != 20 {
        std::process::exit(3);
    }

    let p_from_end = &a[4]; // p_end - 1 points to a[4]
    if p_from_end != p_last {
        std::process::exit(4);
    }
    if *p_from_end != 50 {
        std::process::exit(5);
    }

    let p0_plus_5 = &a[5]; // p0 + 5 points to one past the last element
    if p0_plus_5 != p_end {
        std::process::exit(6);
    }
}