fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = a.as_ptr().add(a.len());

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

    let p_from_end = a.as_ptr().add(a.len() - 1);
    if p_from_end != p_last as *const i32 {
        std::process::exit(4);
    }
    if *p_from_end != 50 {
        std::process::exit(5);
    }

    if a.as_ptr().add(5) != p_end {
        std::process::exit(6);
    }

    std::process::exit(0);
}