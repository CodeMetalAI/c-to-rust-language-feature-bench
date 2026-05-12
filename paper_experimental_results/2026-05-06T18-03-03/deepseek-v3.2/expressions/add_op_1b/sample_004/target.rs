fn main() {
    let mut a = [[0; 3]; 4];
    let mut p: &mut [[i32; 3]] = &mut a;

    // Move to the second row (index 1)
    p = &mut p[1..];
    
    // Set element at row 1, column 2
    p[0][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // Calculate the offset between original and current slice
    let original_ptr = a.as_mut_ptr();
    let current_ptr = p.as_mut_ptr();
    let offset = unsafe { current_ptr.offset_from(original_ptr) } as isize;

    if offset != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}