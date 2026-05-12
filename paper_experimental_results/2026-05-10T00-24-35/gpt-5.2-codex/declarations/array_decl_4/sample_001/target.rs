use std::sync::{Mutex, atomic::{AtomicI32, Ordering}};

static N: i32 = 9;
static BACKING: Mutex<[[i32; 9]; 9]> = Mutex::new([[0; 9]; 9]);
static SINK: AtomicI32 = AtomicI32::new(0);

fn fill_backing() {
    let mut backing = BACKING.lock().unwrap();
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d<F>(m: usize, mut get: F) -> i32
where
    F: FnMut(usize, usize) -> i32,
{
    let mut s: i32 = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= get(i, j) + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &Vec<Vec<i32>>) {
    let mut d = vec![0i32; m];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    let mut backing = BACKING.lock().unwrap();
    for i in 0..m {
        for j in 0..m {
            backing[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    let sum = checksum_2d(m, |i, j| backing[i][j]);
    SINK.fetch_xor(sum, Ordering::SeqCst);
}

fn main() {
    let m = N as usize;

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
        let backing_sum = {
            let backing = BACKING.lock().unwrap();
            checksum_2d(m, |i, j| backing[i][j])
        };
        let expect_sum = checksum_2d(m, |i, j| expect[i][j]);
        if backing_sum != expect_sum {
            std::process::exit(1);
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
        let backing_sum = {
            let backing = BACKING.lock().unwrap();
            checksum_2d(m, |i, j| backing[i][j])
        };
        let expect_sum = checksum_2d(m, |i, j| expect2[i][j]);
        if backing_sum != expect_sum {
            std::process::exit(2);
        }
    }

    if SINK.load(Ordering::SeqCst) == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}