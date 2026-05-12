const N: usize = 9;

fn fill_backing(backing: &mut [[i32; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(a: &[[i32; N]; N]) -> i32 {
    let mut s = 0i32;
    for i in 0..N {
        for j in 0..N {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; N]; N], backing: &mut [[i32; N]; N], sink: &mut i32) {
    let mut d = [0i32; N];
    for i in 0..N {
        d[i] = (i as i32 + 1) * 7 + 3;
    }
    for i in 0..N {
        for j in 0..N {
            let idx = ((i + j) % N) as usize;
            backing[i][j] = c[i][j] + d[idx];
        }
    }
    *sink ^= checksum_2d(backing);
}

fn main() {
    let mut backing = [[0i32; N]; N];
    let mut sink = 0i32;
    fill_backing(&mut backing);
    let mut x = [[0i32; N]; N];
    let mut y = [[0i32; N]; N];
    for i in 0..N {
        for j in 0..N {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }
    fvla(&x, &mut backing, &mut sink);
    {
        let mut expect = [[0i32; N]; N];
        for i in 0..N {
            for j in 0..N {
                let idx = (i + j) % N;
                let d = ((idx as i32) + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(&backing) != checksum_2d(&expect) {
            std::process::exit(1);
        }
    }
    fvla(&y, &mut backing, &mut sink);
    {
        let mut expect2 = [[0i32; N]; N];
        for i in 0..N {
            for j in 0..N {
                let idx = (i + j) % N;
                let d = ((idx as i32) + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(&backing) != checksum_2d(&expect2) {
            std::process::exit(2);
        }
    }
    if sink == 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}