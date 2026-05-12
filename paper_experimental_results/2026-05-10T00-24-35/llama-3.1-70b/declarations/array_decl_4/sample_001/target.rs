fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(backing: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..9 {
        for j in 0..9 {
            s ^= backing[i][j] + (i as i32 * 131) + (j as i32 * 17);
        }
    }
    s
}

fn fvla(backing: &mut [[i32; 9]; 9], C: &[[i32; 9]; 9]) {
    let mut D = [0; 9];
    for i in 0..9 {
        D[i] = (i as i32 + 1) * 7 + 3;
    }

    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = C[i][j] + D[(i + j) % 9];
        }
    }

    sink ^= checksum_2d(backing);
}

static mut sink: i32 = 0;

fn main() {
    let mut backing = [[0; 9]; 9];
    fill_backing(&mut backing);

    let mut X = [[0; 9]; 9];
    let mut Y = [[0; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            X[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            Y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(&mut backing, &X);

    let mut expect = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = (((i + j) % 9) as i32 + 1) * 7 + 3;
            expect[i][j] = X[i][j] + d;
        }
    }
    if checksum_2d(&backing)!= checksum_2d(&expect) {
        std::process::exit(1);
    }

    fvla(&mut backing, &Y);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = (((i + j) % 9) as i32 + 1) * 7 + 3;
            expect2[i][j] = Y[i][j] + d;
        }
    }
    if checksum_2d(&backing)!= checksum_2d(&expect2) {
        std::process::exit(2);
    }

    if unsafe { sink } == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}