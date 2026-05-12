fn main() {
    let mut x: Vec<i32> = vec![0; 15];
    for i in 0..3 {
        for j in 0..5 {
            x[i * 5 + j] = (1 * i + j) as i32;
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
    let p0 = &x[0] as *const i32;
    let p1 = &x[5] as *const i32;
    let diff = unsafe { ((p1 as usize).wrapping_sub(p0 as usize) / std::mem::size_of::<i32>()) as isize };
    if diff != 5 {
        std::process::exit(2);
    }
    println!("PASS");
}