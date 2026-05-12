#[allow(dead_code)]
static mut SINK: i32 = 0;

#[allow(dead_code)]
static BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

#[allow(dead_code)]
fn fill_backing() {
    for i in 0..9 {
        for j in 0..9 {
            BACKING[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}

#[allow(dead_code)]
fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i * 131 + j * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9]) {
    let mut d = vec![0; m];
    for i in 0..m {
        d[i] = (i + 1) * 7 + 3;
    }

    unsafe {
        let q = &mut BACKING;
        for i in 0..m {
            for j in 0..m {
                q[i][j] = c[i][j] + d[(i + j) % m];
            }
        }
    }
}

fn main() {
    let m = 9;

    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(m, &x);

    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        std::process::exit(1);
    }

    fvla(m, &y);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        std::process::exit(2);
    }

    unsafe {
        if SINK == 0 {
            std::process::exit(3);
        } else {
            std::process::exit(0);
        }
    }
}