fn main() {
    let mut x = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            // In Rust, we can simulate pointer arithmetic by flattening the array
            let flat_index = i * 5 + j;
            let flat_slice = &x as *const [[i32; 5]; 3] as *const i32;
            // SAFETY: The index is within bounds of the flattened array (3*5 = 15 elements)
            let b = unsafe { *flat_slice.add(flat_index) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if (p1 as isize - p0 as isize) / std::mem::size_of::<i32>() as isize != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}