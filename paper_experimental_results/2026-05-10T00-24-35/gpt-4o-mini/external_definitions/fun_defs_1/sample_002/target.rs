static mut ARR: [i32; 256] = [0; 256];

fn f(a: u8, b: u8) -> u8 {
    unsafe {
        ARR[b as usize] = 1;
    }
    b
}

fn g(a: i8, b: i8) -> i16 {
    let ub = b as u8;
    unsafe {
        ARR[ub as usize] += b as i32;
    }
    (a as i16) + (b as i16)
}

fn main() -> i32 {
    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a, b);

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
    return 0;
}