fn main() {
    static N: i32 = 9;
    static mut SINK: i32 = 0;

    static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

    fn fill_backing() {
        for i in 0..9 {
            for j in 0..9 {
                unsafe {
                    BACKING[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
                }
            }
        }
    }

    fn checksum_2d<const M: usize>(a: &[[i32; M]; M]) -> i32 {
        let mut s = 0;
        for i in 0..M {
            for j in 0..M {
                s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
            }
        }
        s
    }

    fn fvla<const M: usize>(c: &mut [[i32; M]; M]) {
        let mut d = [0; M];
        for i in 0..M {
            d[i] = (i as i32 + 1) * 7 + 3;
        }

        unsafe {
            for i in 0..M {
                for j in 0..M {
                    BACKING[i][j] = c[i][j] + d[(i + j) % M];
                }
            }

            SINK ^= checksum_2d(&BACKING);
        }
    }

    let m = N as usize;
    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(&mut x);

    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = (((i + j) % m) as i32 + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    unsafe {
        if checksum_2d(&BACKING) != checksum_2d(&expect) {
            std::process::exit(1);
        }
    }

    fvla(&mut y);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = (((i + j) % m) as i32 + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    unsafe {
        if checksum_2d(&BACKING) != checksum_2d(&expect2) {
            std::process::exit(2);
        }
    }

    unsafe {
        if SINK == 0 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}