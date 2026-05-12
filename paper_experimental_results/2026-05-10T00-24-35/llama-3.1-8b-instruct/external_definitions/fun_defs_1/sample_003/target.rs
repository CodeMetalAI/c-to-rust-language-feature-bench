fn f(a: u8, b: u8) -> u8 {
    let mut arr = [0; 256];
    arr[b as usize] = 1;
    b
}

fn g(a: i32, b: i32) -> i32 {
    let mut arr = [0; 256];
    let ub = b as u8;
    arr[ub as usize] += ub as i32;
    a + b
}

fn main() {
    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a, b);

    if r1!= 255 {
        std::process::exit(1);
    }
    if r2!= -1 {
        std::process::exit(2);
    }
    let ub = b as u8;
    if arr[ub as usize]!= 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}