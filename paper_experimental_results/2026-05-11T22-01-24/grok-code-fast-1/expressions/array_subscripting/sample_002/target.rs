fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = (1 * i + j) as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = unsafe {
                let ptr = x.as_ptr().add(i);
                (*ptr)[j]
            };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    let diff = unsafe {
        let p0_ptr = p0 as *const i32;
        let p1_ptr = p1 as *const i32;
        (p1_ptr as usize - p0_ptr as usize) / std::mem::size_of::<i32>()
    };
    if diff as isize != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}