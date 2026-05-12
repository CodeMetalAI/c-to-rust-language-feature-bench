fn main() {
    static mut ARR: [i32; 256] = [0; 256];

    fn f(a: u8, b: u8) -> i32 {
        unsafe {
            ARR[b as usize] = 1;
        }
        b as i32
    }

    fn g(a: i32, b: i32) -> i32 {
        unsafe {
            let idx = b as u8 as usize;
            ARR[idx] += b;
        }
        a + b
    }

    let a = 0i8;
    let b = -1i8;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a as i32, b as i32);

    if r1 != 255 {
        std::process::exit(1);
    }
    if r2 != -1 {
        std::process::exit(2);
    }
    let ub = b as u8 as usize;
    unsafe {
        if ARR[ub] != 0 {
            std::process::exit(3);
        }
    }
    std::process::exit(0);
}