fn main() -> i32 {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // Pointer-like behavior using slice

    p[0][2] = 99;

    if a[1][2] != 99 {
        return 1;
    }

    if (p.as_ptr() as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        return 2;
    }

    0
}