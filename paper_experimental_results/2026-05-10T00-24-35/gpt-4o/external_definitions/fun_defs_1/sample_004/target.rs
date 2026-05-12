static mut ARR: [i32; 256] = [0; 256];

fn f(a: u8, b: u8) -> i32 {
    unsafe {
        ARR[b as usize] = 1;
    }
    b as i32
}

fn g(a: i32, b: i32) -> i32 {
    unsafe {
        ARR[(b as u8) as usize] += b;
    }
    a + b
}

fn main() -> i32 {
    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a as i32, b as i32);

    if r1 != 255 {
        return 1;
    }
    if r2 != -1 {
        return 2;
    }
    let ub = b as u8;
    unsafe {
        if ARR[ub as usize] != 0 {
            return 3;
        }
    }
    0
}