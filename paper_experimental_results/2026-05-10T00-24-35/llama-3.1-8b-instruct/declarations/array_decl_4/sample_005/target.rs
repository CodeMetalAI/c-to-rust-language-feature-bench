const N: usize = 9;

static mut SINK: u32 = 0;

static BACKING: [[u32; N]; N] = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [11, 12, 13, 14, 15, 16, 17, 18, 19],
    [21, 22, 23, 24, 25, 26, 27, 28, 29],
    [31, 32, 33, 34, 35, 36, 37, 38, 39],
    [41, 42, 43, 44, 45, 46, 47, 48, 49],
    [51, 52, 53, 54, 55, 56, 57, 58, 59],
    [61, 62, 63, 64, 65, 66, 67, 68, 69],
    [71, 72, 73, 74, 75, 76, 77, 78, 79],
    [81, 82, 83, 84, 85, 86, 87, 88, 89],
];

fn checksum_2d(m: usize, a: &[[u32; N]; N]) -> u32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i * 131 + j * 17;
        }
    }
    s
}

fn fvla(m: usize, C: &mut [[u32; N]; N]) {
    let mut D = vec![0; m];
    for i in 0..m {
        D[i] = (i + 1) * 7 + 3;
    }

    let mut q = &mut BACKING;
    for i in 0..m {
        for j in 0..m {
            q[i][j] = C[i][j] + D[(i + j) % m];
        }
    }

    SINK ^= checksum_2d(m, &q);
}

fn main() {
    let n = N;
    let mut X = vec![vec![0; N]; N];
    let mut Y = vec![vec![0; N]; N];

    for i in 0..n {
        for j in 0..n {
            X[i][j] = (i + 1) * 10 + (j + 1);
            Y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fill_backing();

    fvla(n, &mut X);

    let mut expect = vec![vec![0; N]; N];
    for i in 0..n {
        for j in 0..n {
            let d = ((i + j) % n + 1) * 7 + 3;
            expect[i][j] = X[i][j] + d;
        }
    }
    if checksum_2d(n, &BACKING)!= checksum_2d(n, &expect) {
        return 1;
    }

    fvla(n, &mut Y);

    let mut expect2 = vec![vec![0; N]; N];
    for i in 0..n {
        for j in 0..n {
            let d = ((i + j) % n + 1) * 7 + 3;
            expect2[i][j] = Y[i][j] + d;
        }
    }
    if checksum_2d(n, &BACKING)!= checksum_2d(n, &expect2) {
        return 2;
    }

    if SINK == 0 {
        return 3;
    }

    return 0;
}

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            BACKING[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}