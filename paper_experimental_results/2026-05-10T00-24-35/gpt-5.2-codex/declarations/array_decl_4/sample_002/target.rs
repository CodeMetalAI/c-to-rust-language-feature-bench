use std::process;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Mutex, OnceLock};

const N: usize = 9;

static SINK: AtomicI32 = AtomicI32::new(0);
static BACKING: OnceLock<Mutex<Vec<Vec<i32>>>> = OnceLock::new();

fn backing() -> &'static Mutex<Vec<Vec<i32>>> {
    BACKING.get_or_init(|| Mutex::new(vec![vec![0; N]; N]))
}

fn fill_backing() {
    let mut b = backing().lock().unwrap();
    for i in 0..N {
        for j in 0..N {
            b[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
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
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    let mut q = backing().lock().unwrap();
    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[(i + j) % m];
        }
    }

    let cs = checksum_2d(m, &q);
    SINK.fetch_xor(cs, Ordering::SeqCst);
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
        let b = backing().lock().unwrap();
        if checksum_2d(m, &b) != checksum_2d(m, &expect) {
            process::exit(1);
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
        let b = backing().lock().unwrap();
        if checksum_2d(m, &b) != checksum_2d(m, &expect2) {
            process::exit(2);
        }
    }

    if SINK.load(Ordering::SeqCst) == 0 {
        process::exit(3);
    }

    process::exit(0);
}