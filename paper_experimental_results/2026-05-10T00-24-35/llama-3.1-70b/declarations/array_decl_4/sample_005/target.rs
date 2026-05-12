fn main() {
    let n = 9;
    let mut backing = [[0; 9]; 9];
    let mut sink = 0;

    fill_backing(&mut backing);

    let mut X = [[0; 9]; 9];
    let mut Y = [[0; 9]; 9];

    fill_XY(&mut X);
    fill_XY(&mut Y);

    fvla(n, &mut X, &mut backing, &mut sink);
    check_checksum(n, &backing);

    fvla(n, &mut Y, &mut backing, &mut sink);
    check_checksum(n, &backing);

    if sink == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}

fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i + 1) * 100 + (j + 1);
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

fn fill_XY(XY: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            XY[i][j] = (i + 1) * 10 + (j + 1);
        }
    }
}

fn fvla(m: usize, C: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9], sink: &mut i32) {
    let mut D = [0; 9];
    for i in 0..m {
        D[i] = (i + 1) * 7 + 3;
    }

    for i in 0..m {
        for j in 0..m {
            backing[i][j] = C[i][j] + D[(i + j) % m];
        }
    }

    *sink ^= checksum_2d(m, backing);
}

fn check_checksum(m: usize, backing: &[[i32; 9]; 9]) {
    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = backing[i][j] - d;
        }
    }
    if checksum_2d(m, backing)!= checksum_2d(m, &expect) {
        std::process::exit(1);
    }
}