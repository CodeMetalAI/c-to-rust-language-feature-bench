fn main() {
    let mut x: Vec<i32> = vec![0; 15];

    for i in 0..3 {
        for j in 0..5 {
            x[i * 5 + j] = i + j;
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
    if (p1_index - p0_index) != 5 {
        std::process::exit(2);
    }

    println!("PASS");
    std::process::exit(0);
}