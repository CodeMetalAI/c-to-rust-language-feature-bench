fn main() {
    let n: i32 = 4;
    let m: usize = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    // Simulate int (*p)[m] = a;
    let mut p_index: usize = 0;

    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n: i32 = p_index as i32;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}