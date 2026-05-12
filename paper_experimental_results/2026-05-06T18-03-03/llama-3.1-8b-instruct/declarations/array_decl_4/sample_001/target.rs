const N: usize = 9;

static mut BACKING: [[i32; N]; N] = [[0; N]; N];

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            BACKING[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; m]; m]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i * 131 + j * 17;
        }
    }
    s
}

fn fvla(m: usize, C: &mut [[i32; m]; m]) {
    let mut D = [0; m];
    for i in 0..m {
        D[i] = (i + 1) * 7 + 3;
    }

    let mut q = &mut BACKING;

    for i in 0..m {
        for j in 0..m {
            q[i][j] = C[i][j] + D[(i + j) % m];
        }
    }

    let mut sink = 0;
    sink ^= checksum_2d(m, &q);
}

fn main() {
    let m = N;

    let mut X: [[i32; m]; m] = [[0; m]; m];
    let mut Y: [[i32; m]; m] = [[0; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            X[i][j] = (i + 1) * 10 + (j + 1);
            Y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(m, &mut X);

    let mut expect = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = X[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        return 1;
    }

    fvla(m, &mut Y);

    let mut expect2 = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect2[i][j] = Y[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        return 2;
    }

    let mut sink = 0;
    if sink == 0 {
        return 3;
    }

    return 0;
}