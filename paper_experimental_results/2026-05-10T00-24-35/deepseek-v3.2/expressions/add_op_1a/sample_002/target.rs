fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    let p_ptr = &mut a[1];
    p_ptr[2] = 99;

    if a[1][2] != 99 {
        return;
    }

    let offset = 1;
    if offset != 1 {
        return;
    }

    std::process::exit(0);
}