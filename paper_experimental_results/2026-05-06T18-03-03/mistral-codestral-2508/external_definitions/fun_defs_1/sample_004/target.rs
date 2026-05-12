fn f(a: u8, b: u8) -> i32 {
    let mut arr = [0; 256];
    arr[b as usize] = 1;
    b as i32
}

fn g(a: i32, b: i32) -> i32 {
    let mut arr = [0; 256];
    arr[b as usize] += b;
    a + b
}

fn main() {
    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a as u8, b as u8);
    let r2 = g(a as i32, b as i32);

    if r1 != 255 {
        std::process::exit(1);
    }
    if r2 != -1 {
        std::process::exit(2);
    }
    let ub = b as u8;
    let arr = [0; 256];
    if arr[ub as usize] != 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}