fn main() {
    let mut a = [[0; 3]; 4];

    // Equivalent of `int (*p)[3] = a;` and `p += 1;`
    let mut p = 1;

    // Equivalent of `(*p)[2] = 99;`
    a[p][2] = 99;

    // Check if `a[1][2] != 99`
    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // Check if `(p - a) != 1`
    if p != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}