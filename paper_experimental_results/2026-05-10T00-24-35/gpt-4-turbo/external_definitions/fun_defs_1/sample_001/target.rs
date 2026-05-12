fn main() {
    let mut arr = [0; 256];

    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a, b, &mut arr);
    let r2 = g(a, b, &mut arr);

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

fn f(a: i8, b: i8, arr: &mut [i32; 256]) -> i32 {
    let a = a as u8;
    let b = b as u8;
    arr[b as usize] = 1;
    b as i32
}

fn g(a: i8, b: i8, arr: &mut [i32; 256]) -> i32 {
    let a = a as i32;
    let b = b as u8;
    arr[b as usize] += b as i32;
    a + b as i32
}