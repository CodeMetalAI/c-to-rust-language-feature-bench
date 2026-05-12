static mut SINK: i32 = 0;
static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    for i in 0..9 {
        for j in 0..9 {
            unsafe {
                BACKING[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            }
        }
    }
}

fn checksum_2d(m: usize, a: &[Vec<i32>]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &mut [Vec<i32>]) {
    let mut d = vec![0; m];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    unsafe {
        for i in 0..m {
            for j in 0..m {
                BACKING[i][j] = c[i][j] + d[(i + j) % m];
            }
        }
        SINK ^= checksum_2d(m, &BACKING);
    }
}

fn main() {
    let n = 9;
    let m = n;

    let mut x = vec![vec![0; m]; m];
    let mut y = vec![vec![0; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(m, &mut x);

    let mut expect = vec![vec![0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    unsafe {
        if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &mut y);

    let mut expect2 = vec![vec![0; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    unsafe {
        if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    unsafe {
        if SINK == 0 {
            std::process::exit(3);
        }
    }

    std::process::exit(0);
}