fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = 1 * i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = unsafe { *(x.as_ptr().add(i).cast::<i32>().add(j)) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if unsafe { p1.offset_from(p0) } != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}