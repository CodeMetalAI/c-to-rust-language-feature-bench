fn main() {
    let mut a = [[0i32; 3]; 4];

    let mut p_index: usize = 0; // represents pointer to a[0]

    p_index += 1;
    a[p_index][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if p_index != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}