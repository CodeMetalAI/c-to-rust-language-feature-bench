fn main() {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a;

    unsafe {
        p = unsafe { &mut *(&a[1..]).get(0).unwrap() };
    }

    p[2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    if (p as *mut [3; i32] as usize - &a as *const [3; i32] as usize) != 1 {
        return 2;
    }

    return 0;
}