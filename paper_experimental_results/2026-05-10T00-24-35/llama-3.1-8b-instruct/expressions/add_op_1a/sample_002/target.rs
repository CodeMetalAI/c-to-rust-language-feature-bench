fn main() {
    let n = 4;
    let m = 3;
    let mut a: [[i32; m]; n] = [[0; m]; n];

    let p = &mut a[1];

    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n = (p as *const _ as usize - &a as *const _ as usize) / a.len().try_into().unwrap() as usize;
    if n != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}