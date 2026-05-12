fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p: &[[i32; 3]] = &a;

    unsafe {
        p = std::ptr::add(p, 1);
        (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        return 1;
    }

    if (p as *const _ as usize - a as *const _ as usize) != 1 * std::mem::size_of::<[[i32; 3]; 3]>() {
        return 2;
    }

    0
}