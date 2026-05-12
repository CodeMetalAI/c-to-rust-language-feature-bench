use std::cell::RefCell;

const N: usize = 9;

thread_local! {
    static BACKING: RefCell<[[i32; N]; N]> = RefCell::new([[0; N]; N]);
    static SINK: RefCell<i32> = RefCell::new(0);
}

fn fill_backing() {
    BACKING.with(|b| {
        let mut backing = b.borrow_mut();
        for i in 0..N {
            for j in 0..N {
                backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            }
        }
    });
}

fn checksum_2d(m: usize, a: &[[i32; N]; N]) -> i32 {
    let mut s: i32 = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; N]; N]) {
    let mut d = vec![0i32; m];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    BACKING.with(|b| {
        let mut backing = b.borrow_mut();
        for i in 0..m {
            for j in 0..m {
                backing[i][j] = c[i][j] + d[(i + j) % m];
            }
        }
        let sum = checksum_2d(m, &*backing);
        SINK.with(|s| {
            let mut sink = s.borrow_mut();
            *sink ^= sum;
        });
    });
}

fn checksum_backing(m: usize) -> i32 {
    BACKING.with(|b| {
        let backing = b.borrow();
        checksum_2d(m, &*backing)
    })
}

fn main() {
    let m = N;

    let mut x = [[0i32; N]; N];
    let mut y = [[0i32; N]; N];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(m, &x);

    {
        let mut expect = [[0i32; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_backing(m) != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = [[0i32; N]; N];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_backing(m) != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    let sink = SINK.with(|s| *s.borrow());
    if sink == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}