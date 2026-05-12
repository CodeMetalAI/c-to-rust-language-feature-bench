fn main() {
    let mut arr: [i32; 256] = [0; 256];
    
    let a: i8 = 0;
    let b: i8 = -1;
    
    // f(a, b) - parameters are unsigned char, so b (-1 as i8) becomes 255 as u8
    let f_b = b as u8;
    arr[f_b as usize] = 1;
    let r1 = f_b as i32;
    
    // g(a, b) - parameters are int, so a and b are sign-extended to i32
    let g_a = a as i32;
    let g_b = b as i32;
    let idx = (g_b as u8) as usize;
    arr[idx] = arr[idx].wrapping_add(g_b as i32);
    let r2 = g_a + g_b;
    
    if r1 != 255 {
        std::process::exit(1);
    }
    if r2 != -1 {
        std::process::exit(2);
    }
    let ub = b as u8;
    if arr[ub as usize] != 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}