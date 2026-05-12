const N: usize = 9;

static mut BACKING: [i32; N * N] = [0; N * N];

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            BACKING[i * N + j] = (i + 1) as i32 * 100 + (j + 1) as i32;
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; m]]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as i32 * 131 + j as i32 * 17);
        }
    }
    s
}

fn fvla(m: usize, C: &mut [[i32; m]]) {
    let mut D = [0; m];
    for i in 0..m {
        D[i] = (i + 1) as i32 * 7 + 3;
    }

    let mut q = BACKING.as_mut_slice();

    for i in 0..m {
        for j in 0..m {
            q[i * m + j] = C[i][j] + D[(i + j) % m];
        }
    }

    let s = checksum_2d(m, q);
    unsafe {
        let mut sink = &mut 0 as *mut i32;
        *sink ^= s;
    }
}

fn main() {
    let m = N;

    let mut X = [[0; m]; m];
    let mut Y = [[0; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            X[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
            Y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
        }
    }

    fvla(m, &mut X);

    let mut expect = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
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
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect2[i][j] = Y[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        return 2;
    }

    let mut sink = 0;
    unsafe {
        if sink == 0 {
            return 3;
        }
    }

    return 0;
}