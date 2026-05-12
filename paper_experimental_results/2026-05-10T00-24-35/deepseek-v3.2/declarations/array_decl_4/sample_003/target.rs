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
        let mut d = [0; M];
        let mut i = 0;
        while i < M {
            d[i] = (i + 1) as i32 * 7 + 3;
            i += 1;
        }

        unsafe {
            let mut q = &mut BACKING[..M][..M];
            i = 0;
            while i < M {
                let mut j = 0;
                while j < M {
                    q[i][j] = c[i][j] + d[(i + j) % M];
                    j += 1;
                }
                i += 1;
            }

            let mut temp = [[0; M]; M];
            for i in 0..M {
                for j in 0..M {
                    temp[i][j] = q[i][j];
                }
            }
            SINK ^= checksum_2d(&temp);
        }
    }

    const N: usize = 9;
    let m = N;

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    fill_backing();

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

    fvla(&x);

    let mut expect = [[0; N]; N];
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
        let mut temp_backing = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                temp_backing[i][j] = BACKING[i][j];
            }
        }
        if checksum_2d(&temp_backing) != checksum_2d(&expect) {
            std::process::exit(1);
        }
    }

    fvla(&y);

    let mut expect2 = [[0; N]; N];
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
        let mut temp_backing = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                temp_backing[i][j] = BACKING[i][j];
            }
        }
        if checksum_2d(&temp_backing) != checksum_2d(&expect2) {
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