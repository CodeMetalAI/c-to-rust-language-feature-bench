fn main() {
    let mut a = [[0; 3]; 4];  // Define a 2D array, initialized with zeroes

    let mut p = &mut a[0];   // Initially point to the first sub-array

    p = &mut a[1];           // Move the pointer to the second sub-array
    p[2] = 99;               // Equivalent to (*p)[2] = 99 in C++

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let offset = p as *const _ as usize - a.as_ptr() as usize;
    let elements_offset = offset / std::mem::size_of_val(&a[0]);
    if elements_offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}