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
            let b = unsafe { *(x.as_ptr().add(i).add(j)) };
            if a != b {
                panic!("a != b");
            }
        }
    }

    let p0 = x[0].as_ptr();
    let p1 = x[1].as_ptr();
    if p1.offset_from(p0) as usize != 5 * 5 {
        panic!("p1 - p0 != 25");
    }

    println!("PASS");
    std::process::exit(0);
}