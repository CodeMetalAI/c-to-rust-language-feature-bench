fn main() {
    let mut a = [[0i32; 3]; 4];

    let mut p: usize = 0;

    p += 1;
    a[p][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    if (p as isize - 0isize) != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}