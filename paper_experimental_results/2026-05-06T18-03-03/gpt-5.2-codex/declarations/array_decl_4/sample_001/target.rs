use std::sync::{Mutex, OnceLock};

static BACKING: OnceLock<Mutex<Vec<Vec<i32>>>> = OnceLock::new();
static SINK: OnceLock<Mutex<i32>> = OnceLock::new();

fn backing() -> &'static Mutex<Vec<Vec<i32>>> {
    BACKING.get_or_init(|| Mutex::new(vec![vec![0; 9]; 9]))
}

fn sink() -> &'static Mutex<i32> {
    SINK.get_or_init(|| Mutex::new(0))
}

fn fill_backing() {
    let mut b = backing().lock().unwrap();
    for i in 0..9 {
        for j in 0..9 {
            b[i][j] = ((i + 1) * 100 + (j + 1)) as i32;
        }
    }
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
        d[i] = ((i + 1) * 7 + 3) as i32;
    }

    let mut q = backing().lock().unwrap();
    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }
    let checksum = checksum_2d(m, &q);
    drop(q);

    let mut s = sink().lock().unwrap();
    *s ^= checksum;
}

fn main() {
    let m: usize = 9;

    let mut x = vec![vec![0i32; m]; m];
    let mut y = vec![vec![0i32; m]; m];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = ((i + 1) * 10 + (j + 1)) as i32;
            y[i][j] = ((i + 1) * 20 + (j + 1)) as i32;
        }
    }

    fvla(m, &x);

    let mut expect = vec![vec![0i32; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = (((i + j) % m) + 1) as i32 * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }

    let backing_checksum = {
        let b = backing().lock().unwrap();
        checksum_2d(m, &b)
    };
    let expect_checksum = checksum_2d(m, &expect);
    if backing_checksum != expect_checksum {
        std::process::exit(1);
    }

    fvla(m, &y);

    let mut expect2 = vec![vec![0i32; m]; m];
    for i in 0..m {
        for j in 0..m {
            let d = (((i + j) % m) + 1) as i32 * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }

    let backing_checksum2 = {
        let b = backing().lock().unwrap();
        checksum_2d(m, &b)
    };
    let expect_checksum2 = checksum_2d(m, &expect2);
    if backing_checksum2 != expect_checksum2 {
        std::process::exit(2);
    }

    let s = *sink().lock().unwrap();
    if s == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}