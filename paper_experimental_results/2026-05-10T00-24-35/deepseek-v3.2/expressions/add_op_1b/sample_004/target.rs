fn main() {
    let mut a = [[0; 3]; 4];
    let mut p = &mut a[0];

    p = &mut a[1];
    p[2] = 99;

    if a[1][2] != 99 {
        return;
    }

    let p_ptr = p as *mut [i32; 3];
    let a_ptr = a.as_mut_ptr();
    let diff = (p_ptr as usize - a_ptr as usize) / std::mem::size_of::<[i32; 3]>();
    if diff != 1 {
        return;
    }
}