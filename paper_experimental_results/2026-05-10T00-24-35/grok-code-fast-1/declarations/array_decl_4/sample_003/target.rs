fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i + 1) * 100 + (j + 1);
        }
    }
}

fn checksum_2d(a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..9 {
        for j in 0..9 {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9], sink: &mut i32) {
    let mut d = [0i32; 9];
    for i in 0..9 {
        d[i] = (i + 1) * 7 + 3;
    }
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = c[i][j] + d[(i + j) % 9];
        }
    }
    *sink ^= checksum_2d(backing);
}

fn main() -> i32 {
    let n = 9;
    let m = n;
    let mut backing = [[0i32; 9]; 9];
    let mut sink = 0i32;
    let mut x = [[0i32; 9]; 9];
    let mut y = [[0i32; 9]; 9];
    fill_backing(&mut backing);
    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }
    fvla(&x, &mut backing, &mut sink);
    {
        let mut expect = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(&backing) != checksum_2d(&expect) {
            return 1;
        }
    }
    fvla(&y, &mut backing, &mut sink);
    {
        let mut expect2 = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(&backing) != checksum_2d(&expect2) {
            return 2;
        }
    }
    if sink == 0 {
        return 3;
    }
    0
}