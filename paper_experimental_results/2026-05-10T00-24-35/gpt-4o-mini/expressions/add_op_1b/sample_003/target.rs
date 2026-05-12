fn main() -> i32 {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let p = &mut a[1]; // p points to the second row

    p[2] = 99; // Set the third column of the second row to 99

    if a[1][2] != 99 {
        return 1; // Exit with code 1 if the condition fails
    }

    if (p.as_ptr() as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        return 2; // Exit with code 2 if the pointer arithmetic fails
    }

    return 0; // Successful execution
}