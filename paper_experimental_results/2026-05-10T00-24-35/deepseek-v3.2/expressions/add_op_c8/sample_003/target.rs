fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = p_last as *const _ as usize + std::mem::size_of::<i32>();

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

    let p_end_computed = &a[5];
    if p_end_computed as *const _ as usize != p_end {
        return std::process::exit(6);
    }

    std::process::exit(0);
}