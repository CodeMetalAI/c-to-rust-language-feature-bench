fn main() {
    static mut arr: [i32; 256] = [0; 256];

    fn f(a: u8, b: u8) -> i32 {
        unsafe { arr[b as usize] = 1 };
        b as i32
    }

    fn g(a: i32, b: i8) -> i32 {
        unsafe { arr[(b as u8) as usize] += b as i32 };
        a + b as i32
    }

    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a as i32, b);

    if r1!= 255 {
        std::process::exit(1);
    }
    if r2!= -1 {
        std::process::exit(2);
    }
    let ub: u8 = b as u8;
    if unsafe { arr[ub as usize] }!= 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}