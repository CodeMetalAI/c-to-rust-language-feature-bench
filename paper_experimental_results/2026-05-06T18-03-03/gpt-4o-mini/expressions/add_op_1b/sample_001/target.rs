fn main() -> i32 {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // Create a mutable slice starting from the second row
    p[0][2] = 99; // Set the value at the second row, third column to 99

    if a[1][2] != 99 {
        return 1; // Return 1 if the value is not 99
    }

    if (p.as_ptr() as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        return 2; // Return 2 if the pointer difference is not 1
    }

    return 0; // Return 0 if all checks pass
}