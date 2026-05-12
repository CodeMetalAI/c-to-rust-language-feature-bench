fn main() {
    let mut arr: [i32; 256] = [0; 256];

    let f = |a: i8, b: i8| {
        let b = b as u8;
        arr[b as usize] = 1;
        b as i32
    };

    let g = |a: i32, b: i32| {
        let b = b as u8;
        arr[b as usize] += b as i32;
        a + b as i32
    };

    let a: i8 = 0;
    let b: i8 = -1;

    let r1 = f(a, b);
    let r2 = g(a as i32, b as i32);

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