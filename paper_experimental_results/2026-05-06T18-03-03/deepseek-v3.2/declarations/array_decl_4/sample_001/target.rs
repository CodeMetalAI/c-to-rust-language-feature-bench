static mut sink: i32 = 0;
static mut backing: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            backing[i][j] = (i + 1) * 100 + (j + 1);
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &[[i32; m]]) -> i32 {
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

fn fvla(m: usize, c: &[[i32; m]]) {
    let mut d = vec![0; m];
    let mut i = 0;
    while i < m {
        d[i] = (i + 1) as i32 * 7 + 3;
        i += 1;
    }

    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            backing[i][j] = c[i][j] + d[(i + j) % m];
            j += 1;
        }
        i += 1;
    }

    sink ^= checksum_2d(m, &backing);
}

fn main() -> i32 {
    let n = 9;
    let m = n;

    let mut x = vec![vec![0; m]; m];
    let mut y = vec![vec![0; m]; m];

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

    let mut expect = vec![vec![0; m]; m];
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
    if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
        return 1;
    }

    fvla(m, &y);

    let mut expect2 = vec![vec![0; m]; m];
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
    if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
        return 2;
    }

    if sink == 0 {
        return 3;
    }

    0
}