fn main() {
    let mut x: Vec<i32> = vec![0; 15];

    for i in 0..3 {
        for j in 0..5 {
            x[i * 5 + j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i * 5 + j];
            let b = x[i * 5 + j];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0_index = 0;
    let p1_index = 1 * 5 + 0;
    if (p1_index as isize - p0_index as isize) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}