fn main() {
    const N: usize = 9;

    static mut SINK: i32 = 0;

    static mut BACKING: [[i32; N]; N] = [[0; N]; N];

    fn fill_backing() {
        for i in 0..N {
            for j in 0..N {
                unsafe { BACKING[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1) }
            }
        }
    }

    fn checksum_2d(m: usize, a: &[[i32; N]; N]) -> i32 {
        let mut s = 0;
        for i in 0..m {
            for j in 0..m {
                s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
            }
        }
        s
    }

    fn fvla(m: usize, c: &[[i32; N]; N]) {
        let mut d = [0; N];
        let q = unsafe { &mut BACKING };

        for i in 0..m {
            d[i] = (i as i32 + 1) * 7 + 3;
        }

        for i in 0..m {
            for j in 0..m {
                q[i][j] = c[i][j] + d[(i + j) % m];
            }
        }

        unsafe { SINK ^= checksum_2d(m, q) }
    }

    fill_backing();

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    for i in 0..N {
        for j in 0..N {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(N, &x);

    let mut expect = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            let d = ((i + j) % N + 1) as i32 * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(N, &BACKING)!= checksum_2d(N, &expect) {
        std::process::exit(1);
    }

    fvla(N, &y);

    let mut expect2 = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            let d = ((i + j) % N + 1) as i32 * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(N, &BACKING)!= checksum_2d(N, &expect2) {
        std::process::exit(2);
    }

    if unsafe { SINK == 0 } {
        std::process::exit(3);
    }
}