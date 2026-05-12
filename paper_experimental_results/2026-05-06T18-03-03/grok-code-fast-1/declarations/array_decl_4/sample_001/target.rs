use std::cell::RefCell;

const N: usize = 9;

static SINK: RefCell<i32> = RefCell::new(0);
static BACKING: RefCell<[[i32; N]; N]> = RefCell::new([[0; N]; N]);

fn fill_backing() {
    let mut backing = BACKING.borrow_mut();
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &[[i32; N]; N]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
            j += 1;
        }
        i += 1;
    }
    s
}

fn fvla(m: usize, c: &[[i32; N]; N]) {
    let mut d = [0; N];
    let mut i = 0;
    while i < m {
        d[i] = (i as i32 + 1) * 7 + 3;
        i += 1;
    }

    let mut backing = BACKING.borrow_mut();

    i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            backing[i][j] = c[i][j] + d[(i + j) % m];
            j += 1;
        }
        i += 1;
    }

    let checksum = checksum_2d(m, &*backing);
    *SINK.borrow_mut() ^= checksum;
}

fn main() -> i32 {
    let m = N;

    let mut x = [[0; N]; N];
    let mut y = [[0; N]; N];

    fill_backing();

    {
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
                y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
                j += 1;
            }
            i += 1;
        }
    }

    fvla(m, &x);

    {
        let mut expect = [[0; N]; N];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = (((i + j) % m) as i32 + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing = BACKING.borrow();
        if checksum_2d(m, &*backing) != checksum_2d(m, &expect) {
            return 1;
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = [[0; N]; N];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = (((i + j) % m) as i32 + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing = BACKING.borrow();
        if checksum_2d(m, &*backing) != checksum_2d(m, &expect2) {
            return 2;
        }
    }

    if *SINK.borrow() == 0 {
        return 3;
    }

    0
}