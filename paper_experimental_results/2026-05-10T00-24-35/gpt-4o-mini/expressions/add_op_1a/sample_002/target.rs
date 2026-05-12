fn main() -> i32 {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let mut p = &mut a[1..]; // Move the pointer to the second row
    p[0][2] = 99; // Set the value at row 1, column 2 to 99

    if a[1][2] != 99 {
        return 1; // Return 1 if the condition fails
    }

    let n = p.as_ptr() as usize / std::mem::size_of::<[i32; 3]>() / std::mem::size_of::<[i32; 3]>();
    if n != 1 {
        return 2; // Return 2 if the condition fails
    }

    return 0; // Return 0 if everything is okay
}