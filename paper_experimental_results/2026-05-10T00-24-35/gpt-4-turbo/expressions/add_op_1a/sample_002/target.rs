fn main() {
    let mut a = [[0; 3]; 4];
    let m = 3;
    
    // Mimicking the behavior of pointer `p` in C++
    let mut p_row = 1;  // Equivalent to `p += 1;` which moves the pointer to the second row of `a`
    a[p_row][2] = 99;  // Equivalent to `(*p)[2] = 99;`

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = p_row;  // Equivalent to `n = (int)(p - a);`
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}