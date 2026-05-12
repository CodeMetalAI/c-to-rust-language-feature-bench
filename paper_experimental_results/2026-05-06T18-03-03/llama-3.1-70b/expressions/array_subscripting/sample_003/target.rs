fn main() {
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];

    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *x.get_unchecked(i).get_unchecked(j);
            if a!= b {
                std::process::exit(1);
            }
        }
    }

    let p0 = x[0].as_ptr();
    let p1 = x[1].as_ptr();
    if (p1 as isize - p0 as isize)!= 5 {
        std::process::exit(2);
    }

    println!("PASS");
}