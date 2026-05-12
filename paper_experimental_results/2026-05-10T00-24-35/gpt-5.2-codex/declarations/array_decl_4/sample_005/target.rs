use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicI32, Ordering};

const N: usize = 9;

static BACKING: OnceLock<Mutex<Vec<Vec<i32>>>> = OnceLock::new();
static SINK: AtomicI32 = AtomicI32::new(0);

fn backing() -> &'static Mutex<Vec<Vec<i32>>> {
    BACKING.get_or_init(|| Mutex::new(vec![vec![0; N]; N]))
}

fn fill_backing() {
    let mut b = backing().lock().unwrap();
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            b[i][j] = ((i as i32) + 1) * 100 + ((j as i32) + 1);
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &Vec<Vec<i32>>) -> i32 {
    let mut s: i32 = 0;
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

fn fvla(m: usize, c: &Vec<Vec<i32>>) {
    let mut d = vec![0i32; m];
    let mut i = 0;
    while i < m {
        d[i] = ((i as i32) + 1) * 7 + 3;
        i += 1;
    }

    let mut q = backing().lock().unwrap();

    i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            q[i][j] = c[i][j] + d[(i + j) % m];
            j += 1;
        }
        i += 1;
    }

    let sum = checksum_2d(m, &q);
    SINK.fetch_xor(sum, Ordering::SeqCst);
}

fn main() {
    let m = N;

    let mut x = vec![vec![0i32; m]; m];
    let mut y = vec![vec![0i32; m]; m];

    fill_backing();

    {
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                x[i][j] = ((i as i32) + 1) * 10 + ((j as i32) + 1);
                y[i][j] = ((i as i32) + 1) * 20 + ((j as i32) + 1);
                j += 1;
            }
            i += 1;
        }
    }

    fvla(m, &x);

    {
        let mut expect = vec![vec![0i32; m]; m];
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
        let backing_sum = {
            let b = backing().lock().unwrap();
            checksum_2d(m, &b)
        };
        if backing_sum != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = vec![vec![0i32; m]; m];
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
        let backing_sum = {
            let b = backing().lock().unwrap();
            checksum_2d(m, &b)
        };
        if backing_sum != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    if SINK.load(Ordering::SeqCst) == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}