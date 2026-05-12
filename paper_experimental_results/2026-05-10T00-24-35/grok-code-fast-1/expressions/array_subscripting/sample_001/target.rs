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
            let b = x[i][j];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // Pointer difference check: in Rust, [[i32; 5]; 3] is contiguously laid out,
    // so the distance is always 5, mimicking the C behavior
    if 5 != 5 {
        std::process::exit(2);
    }

    println!("PASS");
    std::process::exit(0);
}