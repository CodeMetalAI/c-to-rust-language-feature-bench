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
            let b = *(*(x.as_mut_ptr() as *mut [i32; 5] + i) + j);
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as usize - p0 as usize) / std::mem::size_of::<i32>() != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}