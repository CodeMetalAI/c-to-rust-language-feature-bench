fn main() {
    let mut x = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i * 5 + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *(((x.as_ptr() as *const i32).offset(i as isize) as *const [i32]).as_ptr().offset(j as isize));
            if a!= b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if (p1 as isize - p0 as isize)!= 5 {
        std::process::exit(2);
    }

    println!("PASS");
}