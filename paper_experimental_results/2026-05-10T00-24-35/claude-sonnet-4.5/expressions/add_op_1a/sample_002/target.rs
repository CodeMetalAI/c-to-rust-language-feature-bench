fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0i32; 3]; 4];

    let mut p_index = 0;

    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = p_index - 0;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}