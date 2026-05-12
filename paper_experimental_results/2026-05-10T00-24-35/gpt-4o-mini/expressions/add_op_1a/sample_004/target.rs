fn main() -> i32 {
    let n: usize = 4;
    let m: usize = 3;
    let mut a = [[0; 3]; 4];

    let mut p = &mut a as *mut _ as *mut [[i32; 3]];
    unsafe { p = p.add(1) };
    unsafe { (*p)[0][2] = 99 };

    if a[1][2] != 99 {
        return 1;
    }

    let n = unsafe { p.offset_from(&a as *const _ as *const [[i32; 3]]) } as usize;
    if n != 1 {
        return 2;
    }

    return 0;
}