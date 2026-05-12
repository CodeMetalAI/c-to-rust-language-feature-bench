fn main() {
    let n = 9;
    let mut backing: [[i32; 9]; 9];

    fn fill_backing(backing: &mut [[i32; 9]]) {
        for i in 0..9 {
            for j in 0..9 {
                backing[i][j] = (i + 1) * 100 + (j + 1);
            }
        }
    }

    fn checksum_2d(m: usize, a: &[[i32; m]; m]) -> i32 {
        let mut s = 0;
        for i in 0..m {
            for j in 0..m {
                s ^= a[i][j] + i * 131 + j * 17;
            }
        }
        s
    }

    fn fvla(m: usize, c: &mut [[i32; m]; m]) {
        let mut q = backing;
        let mut D: Vec<i32> = (1..m).map(|i| (i + 1) * 7 + 3).collect();

        for i in 0..m {
            for j in 0..m {
                q[i][j] = c[i][j] + D[(i + j) % m];
            }
        }

        let sink = checksum_2d(m, &q);
    }

    fill_backing(&mut backing);

    let m = n;
    let mut X: [[i32; 9]; 9] = [[0; 9]; 9];
    let mut Y: [[i32; 9]; 9] = [[0; 9]; 9];

    for i in 0..m {
        for j in 0..m {
            X[i][j] = (i + 1) * 10 + (j + 1);
            Y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }

    fvla(m, &mut X);

    let mut expect: [[i32; 9]; 9] = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            expect[i][j] = X[i][j] + ((i + j) % m + 1) * 7 + 3;
        }
    }
    if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
        std::process::exit(1);
    }

    fvla(m, &mut Y);

    let mut expect2: [[i32; 9]; 9] = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            expect2[i][j] = Y[i][j] + ((i + j) % m + 1) * 7 + 3;
        }
    }
    if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
        std::process::exit(2);
    }

    if backing[0][0] == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}