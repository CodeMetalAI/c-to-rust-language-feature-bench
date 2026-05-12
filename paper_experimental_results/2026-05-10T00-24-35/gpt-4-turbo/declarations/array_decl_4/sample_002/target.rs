static mut SINK: i32 = 0;
static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    for i in 0..9 {
        for j in 0..9 {
            unsafe {
                BACKING[i][j] = (i + 1) * 100 + (j + 1);
            }
        }
    }
}

fn checksum_2d(matrix: &[[i32; 9]]) -> i32 {
    let mut s = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            s ^= value + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; 9]]) {
    let mut d = [0; 9];
    for i in 0..9 {
        d[i] = (i + 1) * 7 + 3;
    }

    unsafe {
        for i in 0..9 {
            for j in 0..9 {
                BACKING[i][j] = c[i][j] + d[(i + j) % 9];
            }
        }
        SINK ^= checksum_2d(&BACKING);
    }
}

fn main() {
    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    fill_backing();

    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(&x);

    let mut expect = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = ((i + j) % 9 + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }

    if unsafe { checksum_2d(&BACKING) != checksum_2d(&expect) } {
        std::process::exit(1);
    }

    fvla(&y);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = ((i + j) % 9 + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }

    if unsafe { checksum_2d(&BACKING) != checksum_2d(&expect2) } {
        std::process::exit(2);
    }

    if unsafe { SINK == 0 } {
        std::process::exit(3);
    }
}