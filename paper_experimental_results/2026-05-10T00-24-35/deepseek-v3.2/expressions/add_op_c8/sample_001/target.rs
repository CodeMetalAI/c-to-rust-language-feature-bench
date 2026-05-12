fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = unsafe { p0.add(2) };
    let p4 = unsafe { p2.add(2) };
    let p_last = &a[4];
    let p_end = unsafe { p_last.add(1) };

    if unsafe { *p2 } != 30 {
        std::process::exit(1);
    }
    if unsafe { *p4 } != 50 {
        std::process::exit(2);
    }

    let p_back = unsafe { p4.sub(3) };
    if unsafe { *p_back } != 20 {
        std::process::exit(3);
    }

    let p_from_end = unsafe { p_end.sub(1) };
    if !std::ptr::eq(p_from_end, p_last) {
        std::process::exit(4);
    }
    if unsafe { *p_from_end } != 50 {
        std::process::exit(5);
    }

    if unsafe { p0.add(5) } != p_end {
        std::process::exit(6);
    }

    std::process::exit(0);
}