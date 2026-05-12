fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}

fn checksum_2d(matrix: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            s ^= value + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(matrix: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9]) {
    let mut d = [0; 9];
    for i in 0..9 {
        d[i] = (i + 1) * 7 + 3;
    }

    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = matrix[i][j] + d[(i + j) % 9];
        }
    }
}

fn main() {
    let mut backing = [[0; 9]; 9];
    fill_backing(&mut backing);

    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    let mut sink = 0;
    fvla(&x, &mut backing);
    sink ^= checksum_2d(&backing);

    let mut expect = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = ((i + j) % 9 + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }

    if checksum_2d(&backing) != checksum_2d(&expect) {
        std::process::exit(1);
    }

    fvla(&y, &mut backing);
    sink ^= checksum_2d(&backing);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = ((i + j) % 9 + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }

    if checksum_2d(&backing) != checksum_2d(&expect2) {
        std::process::exit(2);
    }

    if sink == 0 {
        std::process::exit(3);
    }
}