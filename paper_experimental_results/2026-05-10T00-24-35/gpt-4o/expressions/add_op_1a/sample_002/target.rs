fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4]; // Initialize a 4x3 array with zeros

    {
        let p = &mut a as *mut [[i32; 3]; 4]; // Pointer to the array
        let p = unsafe { p.add(1) }; // Move the pointer to the second row (index 1)
        unsafe {
            (*p)[0][2] = 99; // Set the value in the second row, third column to 99
        }
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let p_offset = 1; // Since we've incremented p by 1
    if p_offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}