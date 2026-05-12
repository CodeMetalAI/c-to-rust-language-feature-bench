fn main() {
    const N: usize = 9;

    let mut backing: [[i32; N]; N] = [[0; N]; N];
    let mut sink: i32 = 0;

    fill_backing(&mut backing);

    let mut x: [[i32; N]; N] = [[0; N]; N];
    let mut y: [[i32; N]; N] = [[0; N]; N];

    fill_x_y(&mut x, &mut y);

    fvla(N, &x, &mut backing, &mut sink);

    if checksum_2d(N, &backing)!= checksum_2d(N, &expect_x(N)) {
        return;
    }

    fvla(N, &y, &mut backing, &mut sink);

    if checksum_2d(N, &backing)!= checksum_2d(N, &expect_y(N)) {
        return;
    }

    if sink == 0 {
        return;
    }
}

fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9], sink: &mut i32) {
    let mut d = [0; 9];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            backing[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    *sink ^= checksum_2d(m, backing);
}

fn fill_x_y(x: &mut [[i32; 9]; 9], y: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }
}

fn expect_x(m: usize) -> [[i32; 9]; 9] {
    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1) + d;
        }
    }
    expect
}

fn expect_y(m: usize) -> [[i32; 9]; 9] {
    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1) + d;
        }
    }
    expect
}