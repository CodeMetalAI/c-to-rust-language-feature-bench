fn main() {
    let n = 4;
    let m = 3;
    let mut a: [[i32; 3]; 4] = [[0; 3]; 4];

    let mut p: &mut [i32; 3] = &mut a[0];

    p = &mut a[1];
    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = p as *const [i32; 3] as usize - a.as_ptr() as usize;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}