const N: usize = 9;
static mut SINK: u32 = 0;
static mut BACKING: [[u32; N]; N] = [[0; N]; N];

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            BACKING[i][j] = (i as u32 + 1) * 100 + (j as u32 + 1);
        }
    }
}

fn checksum_2d(m: usize, a: &[[u32; m]; m]) -> u32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as u32) * 131 + (j as u32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &mut [[u32; m]; m]) {
    let mut d: [u32; m] = [0; m];
    for i in 0..m {
        d[i] = (i as u32 + 1) * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            c[i][j] += d[(i + j) % m];
        }
    }

    let q = unsafe { &mut *(&mut BACKING as *const _ as *mut [u32; N][N]);
    let mut i = 0;
    while i < m {
        for j in 0..m {
            q[i][j] = c[i][j];
        }
        i += 1;
    }

    unsafe { SINK ^= checksum_2d(m, q) }
}

fn main() {
    let m = N;

    let mut x: [[u32; m]; m] = [[0; m]; m];
    let mut y: [[u32; m]; m] = [[0; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as u32 + 1) * 10 + (j as u32 + 1);
            y[i][j] = (i as u32 + 1) * 20 + (j as u32 + 1);
        }
    }

    fvla(m, &mut x);

    let mut expect: [[u32; m]; m] = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        return 1;
    }

    fvla(m, &mut y);

    let mut expect2: [[u32; m]; m] = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        return 2;
    }

    if SINK == 0 {
        return 3;
    }

    return 0;
}