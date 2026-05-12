fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i as usize][j as usize] = (i as i32) * 1 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i as usize][j as usize];
            let flat = x.as_flatten();
            let b = flat[(i * 5 + j) as usize];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    let diff = (p1 as *const i32 as usize - p0 as *const i32 as usize) / std::mem::size_of::<i32>();
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
    std::process::exit(0);
}