fn main() {
    let x: i32 = 42;
    let p = &x as *const i32;

    if p as usize!= &x as *const i32 as usize {
        return 1;
    }

    let p_end1 = p.offset(1);
    let p_end2 = &x as *const i32 as *mut i32;

    if p_end1 as usize!= p_end2 as usize {
        return 2;
    }

    if p_end1 as usize == p as usize {
        return 3;
    }

    return 0;
}