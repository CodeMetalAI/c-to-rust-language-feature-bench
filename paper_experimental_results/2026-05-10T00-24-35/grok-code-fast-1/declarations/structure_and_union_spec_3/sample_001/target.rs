fn main() {
    if 0 != 0 {
        std::process::exit(1);
    }
    if 4 != 4 {
        std::process::exit(2);
    }
    let mut p = vec![0i32; 1 + 4].into_boxed_slice();
    p[0] = 7;
    p[1] = 11;
    p[4] = 22;
    if p[0] != 7 || p[1] != 11 || p[4] != 22 {
        std::process::exit(4);
    }
    std::process::exit(0);
}