fn main() -> i32 {
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[0..]; // Create a mutable slice of the array

    p = &mut p[1..]; // Move the pointer to the next row
    p[0][2] = 99; // Set the value at the new pointer location

    if a[1][2] != 99 {
        return 1; // Return 1 if the condition fails
    }

    if (p.as_ptr() as usize - a.as_ptr() as usize) / std::mem::size_of::<[i32; 3]>() != 1 {
        return 2; // Return 2 if the condition fails
    }

    return 0; // Return 0 if all checks pass
}