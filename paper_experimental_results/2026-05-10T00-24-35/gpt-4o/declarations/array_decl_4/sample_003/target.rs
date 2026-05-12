static mut SINK: i32 = 0;
static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];
const N: usize = 9;

fn fill_backing() {
    for i in 0..N {
        for j in 0..N {
            unsafe {
                BACKING[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            }
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; N]; N]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; N]; N]) {
    let mut d = [0; N];
    let mut q = [[0; N]; N];

    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    unsafe {
        BACKING = q;
        SINK ^= checksum_2d(m, &BACKING);
    }
}

fn main() -> i32 {
    let m = N;

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(m, &x);

    {
        let mut expect = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d as i32;
            }
        }

        unsafe {
            if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
                return 1;
            }
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d as i32;
            }
        }

        unsafe {
            if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
                return 2;
            }
        }
    }

    unsafe {
        if SINK == 0 {
            return 3;
        }
    }

    0
}