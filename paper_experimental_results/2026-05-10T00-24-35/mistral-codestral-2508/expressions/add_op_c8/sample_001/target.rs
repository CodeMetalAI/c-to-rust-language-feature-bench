fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = &a[5];

    if *p2 != 30 {
        std::process::exit(1);
    }
    if *p4 != 50 {
        std::process::exit(2);
    }

    let p_back = &a[1];
    if *p_back != 20 {
        std::process::exit(3);
    }

    let p_from_end = &a[4];
    if p_from_end != p_last {
        std::process::exit(4);
    }
    if *p_from_end != 50 {
        std::process::exit(5);
    }

    if &a[5] != p_end {
        std::process::exit(6);
    }
}