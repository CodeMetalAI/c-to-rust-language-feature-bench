// array_decl_4.rs

static mut N: i32 = 9;

static mut SINK: i32 = 0;

static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    for i in 0..9 {
        for j in 0..9 {
            BACKING[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}

fn checksum_2d(m: i32, a: &[[i32; 9]]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i * 131 + j * 17;
        }
    }
    s
}

fn fvla(m: i32, c: &mut [[i32; 9]]) {
    let mut d: [i32; 9] = [0; 9];
    for i in 0..m {
        d[i] = (i + 1) * 7 + 3;
    }

    let mut q = BACKING.as_mut();
    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    unsafe {
        SINK ^= checksum_2d(m, &BACKING);
    }
}

fn main() {
    let m = N;

    let mut x: [[i32; 9]; 9] = [[0; 9]; 9];
    let mut y: [[i32; 9]; 9] = [[0; 9]; 9];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(m, &mut x);

    let mut expect: [[i32; 9]; 9] = [[0; 9]; 9];
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

    let mut expect2: [[i32; 9]; 9] = [[0; 9]; 9];
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