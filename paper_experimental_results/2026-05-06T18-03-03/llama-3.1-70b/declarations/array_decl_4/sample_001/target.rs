fn main() {
    const N: usize = 9;

    let mut backing: [[i32; N]; N] = [[0; N]; N];

    fn fill_backing(backing: &mut [[i32; N]; N]) {
        for i in 0..N {
            for j in 0..N {
                backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
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

    fn fvla(m: usize, c: &[[i32; N]; N], backing: &mut [[i32; N]; N]) {
        let mut d: [i32; N] = [0; N];
        for i in 0..m {
            d[i] = (i as i32 + 1) * 7 + 3;
        }

        for i in 0..m {
            for j in 0..m {
                backing[i][j] = c[i][j] + d[(i + j) % m];
            }
        }

        let mut sink = 0;
        sink ^= checksum_2d(m, backing);
    }

    fill_backing(&mut backing);

    let mut x: [[i32; N]; N] = [[0; N]; N];
    let mut y: [[i32; N]; N] = [[0; N]; N];

    for i in 0..N {
        for j in 0..N {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(N, &x, &mut backing);

    let mut expect: [[i32; N]; N] = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            let d = ((i + j) % N + 1) as i32 * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(N, &backing)!= checksum_2d(N, &expect) {
        std::process::exit(1);
    }

    fvla(N, &y, &mut backing);

    let mut expect2: [[i32; N]; N] = [[0; N]; N];
    for i in 0..N {
        for j in 0..N {
            let d = ((i + j) % N + 1) as i32 * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(N, &backing)!= checksum_2d(N, &expect2) {
        std::process::exit(2);
    }

    if checksum_2d(N, &backing) == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}