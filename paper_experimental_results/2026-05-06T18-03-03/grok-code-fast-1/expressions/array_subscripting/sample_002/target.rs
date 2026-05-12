fn main() {
    let mut x: [i32; 15] = [0; 15];

    for i in 0..3 {
        for j in 0..5 {
            x[i * 5 + j] = (1 * i as i32) + (j as i32);
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

    let diff = 5;
    if diff != 5 {
        std::process::exit(2);
    }

    println!("PASS");
}