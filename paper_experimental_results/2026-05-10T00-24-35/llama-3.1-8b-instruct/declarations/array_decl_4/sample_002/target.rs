const N: usize = 9;

static mut SINK: i32 = 0;

static mut BACKING: [[i32; N]; N] = [[0; N]; N];

static mut EXPECT_X: [[i32; N]; N] = [[0; N]; N];
static mut EXPECT_Y: [[i32; N]; N] = [[0; N]; N];

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            BACKING[i][j] = (i + 1) as i32 * 100 + (j + 1) as i32;
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; m]]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(m: usize, C: &[[i32; m]]) {
    let mut D = [0 as i32; m];
    for i in 0..m {
        D[i] = (i + 1) as i32 * 7 + 3;
    }

    let mut q = &BACKING;

    for i in 0..m {
        for j in 0..m {
            q[i][j] = C[i][j] + D[(i + j) % m];
        }
    }

    unsafe {
        SINK ^= checksum_2d(m, &q);
    }
}

fn main() {
    let m = N;

    let mut X = [[0; N]; N];
    let mut Y = [[0; N]; N];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            X[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
            Y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
        }
    }

    fvla(m, &X);

    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            EXPECT_X[i][j] = X[i][j] + d;
        }
    }

    if checksum_2d(m, &BACKING) != checksum_2d(m, &EXPECT_X) {
        return 1;
    }

    fvla(m, &Y);

    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            EXPECT_Y[i][j] = Y[i][j] + d;
        }
    }

    if checksum_2d(m, &BACKING) != checksum_2d(m, &EXPECT_Y) {
        return 2;
    }

    if SINK == 0 {
        return 3;
    }

    return 0;
}