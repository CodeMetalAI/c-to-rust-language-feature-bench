use std::cell::{Cell, RefCell};
use std::process::exit;

const N: usize = 9;

thread_local! {
    static SINK: Cell<i32> = Cell::new(0);
    static BACKING: RefCell<Vec<Vec<i32>>> = RefCell::new(vec![vec![0; N]; N]);
}

fn fill_backing() {
    BACKING.with(|b| {
        let mut b = b.borrow_mut();
        for i in 0..N {
            for j in 0..N {
                b[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            }
        }
    });
}

fn checksum_2d(m: usize, a: &Vec<Vec<i32>>) -> i32 {
    let mut s: i32 = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &Vec<Vec<i32>>) {
    let mut d = vec![0i32; m];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    BACKING.with(|b| {
        let mut b = b.borrow_mut();
        for i in 0..m {
            for j in 0..m {
                b[i][j] = c[i][j] + d[(i + j) % m];
            }
        }
    });

    let chk = BACKING.with(|b| {
        let b = b.borrow();
        checksum_2d(m, &b)
    });

    SINK.with(|s| {
        s.set(s.get() ^ chk);
    });
}

fn main() {
    let m = N;

    let mut x = vec![vec![0i32; m]; m];
    let mut y = vec![vec![0i32; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(m, &x);

    {
        let mut expect = vec![vec![0i32; m]; m];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        let chk_backing = BACKING.with(|b| {
            let b = b.borrow();
            checksum_2d(m, &b)
        });
        if chk_backing != checksum_2d(m, &expect) {
            exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = vec![vec![0i32; m]; m];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        let chk_backing = BACKING.with(|b| {
            let b = b.borrow();
            checksum_2d(m, &b)
        });
        if chk_backing != checksum_2d(m, &expect2) {
            exit(2);
        }
    }

    let sink_val = SINK.with(|s| s.get());
    if sink_val == 0 {
        exit(3);
    }

    exit(0);
}