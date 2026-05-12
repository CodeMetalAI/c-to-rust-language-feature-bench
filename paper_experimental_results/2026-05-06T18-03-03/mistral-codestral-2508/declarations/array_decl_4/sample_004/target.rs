fn main() {
    let n = 9;
    let mut sink = 0;

    let mut backing = [[0; 9]; 9];

    fn fill_backing(backing: &mut [[i32; 9]; 9]) {
        for i in 0..9 {
            for j in 0..9 {
                backing[i][j] = (i + 1) as i32 * 100 + (j + 1) as i32;
            }
        }
    }

    fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
        let mut s = 0;
        for i in 0..m {
            for j in 0..m {
                s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
            }
        }
        s
    }

    fn fvla(m: usize, c: &mut [[i32; 9]; 9], backing: &mut [[i32; 9]; 9], sink: &mut i32) {
        let mut d = [0; 9];
        let s = c;
        let mut q = backing;

        for i in 0..m {
            d[i] = (i + 1) as i32 * 7 + 3;
        }

        for i in 0..m {
            for j in 0..m {
                q[i][j] = s[i][j] + d[(i + j) % m];
            }
        }

        *sink ^= checksum_2d(m, q);
    }

    let m = n;

    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    fill_backing(&mut backing);

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
            y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
        }
    }

    fvla(m, &mut x, &mut backing, &mut sink);

    let mut expect = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
        std::process::exit(1);
    }

    fvla(m, &mut y, &mut backing, &mut sink);

    let mut expect2 = [[0; 9]; 9];
    for i in 0..m {
        for j in 0..m {
            let d = ((i + j) % m + 1) as i32 * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
        std::process::exit(2);
    }

    if sink == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}