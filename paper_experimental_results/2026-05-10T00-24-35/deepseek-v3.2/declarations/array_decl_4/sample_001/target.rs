static N: usize = 9;
static mut SINK: i32 = 0;
static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            BACKING[i][j] = (i + 1) * 100 + (j + 1);
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &[[i32; m]; m]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
            j += 1;
        }
        i += 1;
    }
    s
}

fn fvla(m: usize, c: &[[i32; m]; m]) {
    let mut d = [0; m];
    let mut i = 0;
    while i < m {
        d[i] = (i + 1) as i32 * 7 + 3;
        i += 1;
    }

    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            BACKING[i][j] = c[i][j] + d[(i + j) % m];
            j += 1;
        }
        i += 1;
    }

    SINK ^= checksum_2d(m, &BACKING);
}

fn main() {
    let m = N;

    let mut x = [[0; m]; m];
    let mut y = [[0; m]; m];

    fill_backing();

    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            x[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
            y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
            j += 1;
        }
        i += 1;
    }

    fvla(m, &x);

    let mut expect = [[0; m]; m];
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect[i][j] = x[i][j] + d;
            j += 1;
        }
        i += 1;
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        return std::process::exit(1);
    }

    fvla(m, &y);

    let mut expect2 = [[0; m]; m];
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect2[i][j] = y[i][j] + d;
            j += 1;
        }
        i += 1;
    }
    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        return std::process::exit(2);
    }

    if SINK == 0 {
        return std::process::exit(3);
    }

    std::process::exit(0);
}