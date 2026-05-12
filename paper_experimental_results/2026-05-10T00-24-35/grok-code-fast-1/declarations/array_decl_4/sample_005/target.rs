use std::cell::RefCell;

const N: usize = 9;

static BACKING: RefCell<[[i32; N]; N]> = RefCell::new([[0; N]; N]);
static SINK: RefCell<i32> = RefCell::new(0);

fn fill_backing() {
    let mut b = BACKING.borrow_mut();
    for i in 0..N {
        for j in 0..N {
            b[i][j] = (i + 1) as i32 * 100 + (j + 1) as i32;
        }
    }
}

fn checksum_2d(arr: &[[i32; N]; N]) -> i32 {
    let mut s = 0;
    for i in 0..N {
        for j in 0..N {
            s ^= arr[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; N]; N]) {
    let mut d = [0i32; N];
    for i in 0..N {
        d[i] = (i + 1) as i32 * 7 + 3;
    }
    let mut b = BACKING.borrow_mut();
    for i in 0..N {
        for j in 0..N {
            b[i][j] = c[i][j] + d[((i + j) % N) as usize];
        }
    }
    let ch = checksum_2d(&*b);
    *SINK.borrow_mut() ^= ch;
}

fn main() -> i32 {
    let m = N;
    let mut x: [[i32; N]; N] = [[0; N]; N];
    let mut y: [[i32; N]; N] = [[0; N]; N];
    fill_backing();
    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i + 1) as i32 * 10 + (j + 1) as i32;
            y[i][j] = (i + 1) as i32 * 20 + (j + 1) as i32;
        }
    }
    fvla(&x);
    {
        let mut expect: [[i32; N]; N] = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(&BACKING.borrow()) != checksum_2d(&expect) {
            return 1;
        }
    }
    fvla(&y);
    {
        let mut expect2: [[i32; N]; N] = [[0; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(&BACKING.borrow()) != checksum_2d(&expect2) {
            return 2;
        }
    }
    if *SINK.borrow() == 0 {
        return 3;
    }
    0
}