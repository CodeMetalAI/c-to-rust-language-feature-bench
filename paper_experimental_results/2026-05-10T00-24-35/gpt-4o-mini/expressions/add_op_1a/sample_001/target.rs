fn main() -> i32 {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let p = &mut a[1..]; // Create a mutable slice starting from the second row
    p[0][2] = 99; // Set the value at the second row and third column to 99

    if a[1][2] != 99 {
        return 1; // Check if the value is as expected
    }

    let n = p.len(); // Get the length of the slice starting from the second row
    if n != 3 {
        return 2; // Check if the offset corresponds to the second row
    }

    return 0; // Return 0 on success
}