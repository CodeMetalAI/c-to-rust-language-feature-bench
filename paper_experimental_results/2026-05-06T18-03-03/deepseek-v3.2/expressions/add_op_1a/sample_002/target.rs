fn main() {
    let n = 4;
    let m = 3;
    let mut a = [[0; 3]; 4];

    // Simulate pointer arithmetic by using slices and indices
    let mut row_index = 0;
    let col_count = m;

    // p += 1
    row_index += 1;
    
    // (*p)[2] = 99
    a[row_index][2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // n = (int)(p - a)
    let n_diff = row_index as i32 - 0; // p - a in pointer arithmetic terms

    if n_diff != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}