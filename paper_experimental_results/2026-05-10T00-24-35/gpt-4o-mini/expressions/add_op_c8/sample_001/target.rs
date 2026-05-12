fn main() -> i32 {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = unsafe { &a[2] }; // Simulating pointer arithmetic
    let p4 = unsafe { &a[4] }; // Simulating pointer arithmetic
    let p_last = &a[4];
    let p_end = unsafe { &a[5] }; // Simulating pointer arithmetic

    if *p2 != 30 {
        return 1;
    }
    if *p4 != 50 {
        return 2;
    }

    let p_back = unsafe { &a[1] }; // Simulating pointer arithmetic
    if *p_back != 20 {
        return 3;
    }

    let p_from_end = unsafe { &a[4] }; // Simulating pointer arithmetic
    if p_from_end != p_last {
        return 4;
    }
    if *p_from_end != 50 {
        return 5;
    }

    if (p0 as *const _ as usize + 5 * std::mem::size_of::<i32>()) as *const i32 != p_end {
        return 6;
    }

    return 0;
}