fn main() {
    let mut arr: [i32; 256] = [0; 256];

    let a: i8 = 0;
    let b: i8 = -1;

    // f(a, b) - parameters are unsigned char, so b (-1) becomes 255
    let f_a: u8 = a as u8;
    let f_b: u8 = b as u8;
    let _ = f_a;
    arr[f_b as usize] = 1;
    let r1: i32 = f_b as i32;

    // g(a, b) - parameters are int, so a and b are sign-extended
    let g_a: i32 = a as i32;
    let g_b: i32 = b as i32;
    // arr[(unsigned char)b] += b means arr[255] += -1
    let idx: u8 = g_b as u8;
    arr[idx as usize] += g_b;
    let r2: i32 = g_a + g_b;

    if r1 != 255 {
        std::process::exit(1);
    }
    if r2 != -1 {
        std::process::exit(2);
    }
    let ub: u8 = b as u8;
    if arr[ub as usize] != 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}