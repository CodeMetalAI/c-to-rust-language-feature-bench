fn main() -> i32 {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // This effectively shifts the pointer.

    p[0][2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    let n = p.as_ptr() as usize / std::mem::size_of::<[i32; 3]>() - a.as_ptr() as usize / std::mem::size_of::<[i32; 3]>();
    if n != 1 {
        return 2;
    }

    return 0;
}