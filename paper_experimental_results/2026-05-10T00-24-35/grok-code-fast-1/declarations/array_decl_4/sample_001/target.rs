use std::cell::RefCell;

const N: usize = 9;

static SINK: RefCell<i32> = RefCell::new(0);

fn fill_backing(backing: &mut [[i32; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(_m: usize, a: &[[i32; N]; N]) -> i32 {
    let mut s = 0;
    for i in 0..N {
        for j in 0..N {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, q: &mut [[i32; N]; N], c: &[[i32; N]; N]) {
    let mut d = vec![0; m];
    for i in 0..m {
        d[i] = ((i + 1) as i32) * 7 + 3;
    }
    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }
    *SINK.borrow_mut() ^= checksum_2d(m, q);
}

fn main() {
    let m = N;
    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];
    for i in 0..m {
        for j in 0..m {
            x[i][j] = ((i + 1) as i32) * 10 + (j + 1) as i32;
            y[i][j] = ((i + 1) as i32) * 20 + (j + 1) as i32;
        }
    }
    let mut backing = [[0; N]; N];
    fill_backing(&mut backing);
    fvla(m, &mut backing, &x);
    let mut expect = [[0; N]; N];
    for i in 0..m {
        for j in 0..m {
            let d = (((i + j) % m + 1) as i32) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
        std::process::exit(1);
    }
    fvla(m, &mut backing, &y);
    let mut expect2 = [[0; N]; N];
    for i in 0..m {
        for j in 0..m {
            let d = (((i + j) % m + 1) as i32) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
        std::process::exit(2);
    }
    if *SINK.borrow() == 0 {
        std::process::exit(3);
    }
    std::process::exit(0);
}