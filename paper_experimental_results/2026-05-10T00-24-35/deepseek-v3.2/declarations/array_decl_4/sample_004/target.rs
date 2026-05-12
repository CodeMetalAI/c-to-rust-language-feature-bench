static mut SINK: i32 = 0;
static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    for i in 0..9 {
        for j in 0..9 {
            BACKING[i][j] = (i + 1) * 100 + (j + 1);
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

fn fvla(m: usize, c: &[[i32; m]]) {
    let mut d = vec![0; m];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            BACKING[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    SINK ^= checksum_2d(m, &BACKING);
}

fn main() {
    let n = 9;
    let m = n;

    let mut x = [[0; m]; m];
    let mut y = [[0; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][1] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][1] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(m, &x);

    let mut expect = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect[i][1] = x[i][1] + d;
        }
    }

    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        return;
    }

    fvla(m, &y);

    let mut expect2 = [[0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect2[i][1] = y[i][1] + d;
        }
    }

    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        return;
    }

    if SINK == 0 {
        return;
    }

    return;
}