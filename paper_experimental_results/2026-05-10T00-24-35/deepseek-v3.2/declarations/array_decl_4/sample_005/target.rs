fn main() {
    static mut SINK: i32 = 0;
    static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

    fn fill_backing() {
        unsafe {
            let mut i = 0;
            while i < 9 {
                let mut j = 0;
                while j < 9 {
                    BACKING[i][j] = (i + 1) as i32 * 100 + (j + 1) as i32;
                    j += 1;
                }
                i += 1;
            }
        }
    }

    fn checksum_2d<const M: usize>(a: &[[i32; M]; M]) -> i32 {
        let mut s = 0;
        let mut i = 0;
        while i < M {
            let mut j = 0;
            while j < M {
                s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
                j += 1;
            }
            i += 1;
        }
        s
    }

    fn fvla<const M: usize>(c: &[[i32; M]; M]) {
        unsafe {
            let mut d = [0; M];
            let mut i = 0;
            while i < M {
                d[i] = (i + 1) as i32 * 7 + 3;
                i += 1;
            }

            i = 0;
            while i < M {
                let mut j = 0;
                while j < M {
                    BACKING[i][j] = c[i][j] + d[(i + j) % M];
                    j += 1;
                }
                i += 1;
            }

            SINK ^= checksum_2d(&BACKING);
        }
    }

    const N: usize = 9;
    let m = N;

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    fill_backing();

    {
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                x[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
                y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
                j += 1;
            }
            i += 1;
        }
    }

    fvla(&x);

    {
        let mut expect = [[0; 9]; 9];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect[i][j] = x[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        unsafe {
            if checksum_2d(&BACKING) != checksum_2d(&expect) {
                std::process::exit(1);
            }
        }
    }

    fvla(&y);

    {
        let mut expect2 = [[0; 9]; 9];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        unsafe {
            if checksum_2d(&BACKING) != checksum_2d(&expect2) {
                std::process::exit(2);
            }
        }
    }

    unsafe {
        if SINK == 0 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}