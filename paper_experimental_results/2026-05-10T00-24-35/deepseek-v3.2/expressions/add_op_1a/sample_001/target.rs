fn main() {
    const N: usize = 4;
    const M: usize = 3;
    let mut a = [[0; M]; N];
    
    let mut p: &mut [[i32; M]] = &mut a;
    
    // Move pointer to second row (index 1)
    p = &mut p[1..];
    
    // Set element at index [0][2] in current slice view (which corresponds to a[1][2])
    p[0][2] = 99;
    
    if a[1][2] != 99 {
        std::process::exit(1);
    }
    
    // Calculate offset: pointer arithmetic equivalent
    // In original: n = (int)(p - a) where p points to row 1
    // With slices: p currently starts at row 1, so offset = original array len - current slice len
    let offset = N - p.len();
    if offset != 1 {
        std::process::exit(2);
    }
    
    std::process::exit(0);
}