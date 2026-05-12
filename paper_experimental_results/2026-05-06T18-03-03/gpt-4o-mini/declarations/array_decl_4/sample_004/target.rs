static mut SINK: i32 = 0;

const N: usize = 9;
static mut BACKING: [[i32; N]; N] = [[0; N]; N];

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            unsafe {
                BACKING[i][j] = (i + 1) as i32 * 100 + (j + 1) as i32;
            }
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
    let q: &mut [[i32; N]; N] = unsafe { &mut BACKING };

    for i in 0..m {
        d[i] = (i + 1) as i32 * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    unsafe {
        SINK ^= checksum_2d(m, q);
    }
}

fn main() -> i32 {
    let m = N;

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
            y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
        }
    }

    fvla(m, &x);

    {
        let mut expect = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(m, unsafe { &BACKING }) != checksum_2d(m, &expect) {
            return 1;
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(m, unsafe { &BACKING }) != checksum_2d(m, &expect2) {
            return 2;
        }
    }

    unsafe {
        if SINK == 0 {
            return 3;
        }
    }

    0
}