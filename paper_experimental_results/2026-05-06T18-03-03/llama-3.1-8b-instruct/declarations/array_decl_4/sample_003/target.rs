const N: usize = 9;

static mut SINK: i32 = 0;

static mut BACKING: [[[i32; N]; N]; N] = [[0; N]; N; N];

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

fn fvla(m: usize, c: &mut [[i32; m]; m]) {
    let mut d = [0; m];
    for i in 0..m {
        d[i] = (i + 1) * 7 + 3;
    }

    let mut q = &mut BACKING;
    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    unsafe {
        SINK ^= checksum_2d(m, &q);
    }
}

fn main() {
    let m = N;

    let mut x = [[0; m]; m];
    let mut y = [[0; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(m, &mut x);

    let mut expect = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        panic!("Checksum mismatch");
    }

    fvla(m, &mut y);

    let mut expect2 = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        panic!("Checksum mismatch");
    }

    if SINK == 0 {
        panic!("Sink is zero");
    }
}