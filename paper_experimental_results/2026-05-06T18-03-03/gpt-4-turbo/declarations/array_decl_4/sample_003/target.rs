fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}

fn checksum_2d(a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for (i, row) in a.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            s ^= value + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9]) {
    let mut d = [0; 9];
    for i in 0..m {
        d[i] = (i + 1) * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            backing[i][j] = c[i][j] + d[(i + j) % m];
        }
    }
}

fn main() {
    let m = 9;
    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];
    let mut backing = [[0; 9]; 9];

    fill_backing(&mut backing);

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(m, &x, &mut backing);

    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(&backing) != checksum_2d(&expect) {
        std::process::exit(1);
    }

    fvla(m, &y, &mut backing);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(&backing) != checksum_2d(&expect2) {
        std::process::exit(2);
    }

    let mut sink = 0;
    sink ^= checksum_2d(&backing);
    if sink == 0 {
        std::process::exit(3);
    }
}