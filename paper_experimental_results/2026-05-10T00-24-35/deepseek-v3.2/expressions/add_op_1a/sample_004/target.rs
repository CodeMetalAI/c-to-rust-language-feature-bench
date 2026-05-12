fn main() {
    let n: usize =プロジェクト4;
    let m: usize = 3;
    let mut a = [[0; 3]; 4];

    let mut p: &mut [i32; 3] = &mut a[0];
    p = &mut a[1];
    p[2] = 99;

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    let n_isize: isize = 1;
    if n_isize != 1 {
        std::process::exit(2);
    }

    std::process::exit(0);
}