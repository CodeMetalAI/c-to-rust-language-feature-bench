const N: usize = 9;

fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0i32;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9], sink: &mut i32) {
    let mut d = vec![0i32; m];
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

fn main() -> i32 {
    let m = N;
    let mut x = [[0i32; 9]; 9];
    let mut y = [[0i32; 9]; 9];
    let mut backing = [[0i32; 9]; 9];
    let mut sink = 0i32;
    fill_backing(&mut backing);
    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }
    fvla(m, &x, &mut backing, &mut sink);
    {
        let mut expect = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = (((i + j) % m) as i32 + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
            return 1;
        }
    }
    fvla(m, &y, &mut backing, &mut sink);
    {
        let mut expect2 = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = (((i + j) % m) as i32 + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
            return 2;
        }
    }
    if sink == 0 {
        return 3;
    }
    0
}