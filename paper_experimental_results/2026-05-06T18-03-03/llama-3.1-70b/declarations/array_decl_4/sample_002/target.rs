fn main() {
    const N: usize = 9;

    let mut backing = [[0; N]; N];
    let mut sink = 0;

    fill_backing(&mut backing);

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    init_arrays(&mut x, &mut y);

    fvla(N, &x, &mut backing, &mut sink);

    let mut expect = [[0; N]; N];
    init_expect(&x, &mut expect);

    if checksum_2d(N, &backing)!= checksum_2d(N, &expect) {
        std::process::exit(1);
    }

    fvla(N, &y, &mut backing, &mut sink);

    let mut expect2 = [[0; N]; N];
    init_expect(&y, &mut expect2);

    if checksum_2d(N, &backing)!= checksum_2d(N, &expect2) {
        std::process::exit(2);
    }

    if sink == 0 {
        std::process::exit(3);
    }
}

fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
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

fn fvla(m: usize, c: &[[i32; 9]; 9], backing: &mut [[i32; 9]; 9], sink: &mut i32) {
    let mut d = [0; 9];
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

fn init_arrays(x: &mut [[i32; 9]; 9], y: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }
}

fn init_expect(a: &[[i32; 9]; 9], expect: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            let d = (((i + j) % 9 + 1) as i32) * 7 + 3;
            expect[i][j] = a[i][j] + d;
        }
    }
}