fn main() {
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];
    let mut p_offset = 0; // p starts at a (which is &a[0])
    p_offset += 1;
    a[p_offset][2] = 99;
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    if p_offset != 1 {
        std::process::exit(2);
    }
    std::process::exit(0);
}