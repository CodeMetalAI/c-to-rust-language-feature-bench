fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let mut p_index: usize = 0;
    p_index += 1;
    a[p_index][2] = 99;
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    let diff = p_index as isize - 0isize;
    if diff != 1 {
        std::process::exit(2);
    }
    std::process::exit(0);
}