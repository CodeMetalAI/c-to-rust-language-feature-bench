fn main() {
    let mut arr = [0; 256];

    let a = 0i8;
    let b = -1i8;

    let r1 = f(a as u8, b as u8, &mut arr);
    let r2 = g(a as i32, b as i32, &mut arr);

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
}

fn f(a: u8, b: u8, arr: &mut [i32; 256]) -> i32 {
    arr[b as usize] = 1;
    b as i32
}

fn g(a: i32, b: i32, arr: &mut [i32; 256]) -> i32 {
    let ub = b as u8;
    arr[ub as usize] += b;
    a + b
}