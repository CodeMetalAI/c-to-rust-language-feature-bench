fn main() -> i32 {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // p points to the second row

    p[0][2] = 99; // Set the third element of the second row to 99

    if a[1][2] != 99 {
        return 1; // Return 1 if the value is not 99
    }

    if (p.as_ptr() as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        return 2; // Return 2 if the pointer arithmetic is not correct
    }

    return 0; // Return 0 if everything is fine
}