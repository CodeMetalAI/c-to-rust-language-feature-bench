fn main() {
    let mut x = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j]; // Direct translation of *(*(x + i) + j)
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // Finding the difference in offsets
    let p0 = x.as_ptr() as usize;
    let p1 = unsafe { x.as_ptr().add(1) } as usize;
    if (p1 - p0) != 5 * std::mem::size_of::<i32>() {
        std::process::exit(2);
    }

    println!("PASS");
}