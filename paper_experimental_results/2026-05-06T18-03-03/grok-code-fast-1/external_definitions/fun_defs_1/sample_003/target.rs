fn f(arr: &mut [i32; 256], _a: u8, b: u8) -> i32 {
    arr[b as usize] = 1;
    b as i32
}

fn g(arr: &mut [i32; 256], a: i32, b: i32) -> i32 {
    let idx = (b as u8) as usize;
    arr[idx] += b;
    a + b
}

fn main() {
    let mut arr = [0i32; 256];
    let a: i8 = 0;
    let b: i8 = -1;
    let r1 = f(&mut arr, a as u8, b as u8);
    let r2 = g(&mut arr, a as i32, b as i32);
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