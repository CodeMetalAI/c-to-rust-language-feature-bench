use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicI32, Ordering};

static N: i32 = 9;
static SINK: AtomicI32 = AtomicI32::new(0);
static BACKING: OnceLock<Mutex<Vec<Vec<i32>>>> = OnceLock::new();

fn backing() -> &'static Mutex<Vec<Vec<i32>>> {
    BACKING.get_or_init(|| Mutex::new(vec![vec![0; 9]; 9]))
}

fn fill_backing() {
    let mut b = backing().lock().unwrap();
    let mut i = 0usize;
    while i < 9 {
        let mut j = 0usize;
        while j < 9 {
            b[i][j] = ((i + 1) as i32).wrapping_mul(100).wrapping_add((j + 1) as i32);
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &Vec<Vec<i32>>) -> i32 {
    let mut s: i32 = 0;
    let mut i = 0usize;
    while i < m {
        let mut j = 0usize;
        while j < m {
            let term = a[i][j]
                .wrapping_add((i as i32).wrapping_mul(131))
                .wrapping_add((j as i32).wrapping_mul(17));
            s ^= term;
            j += 1;
        }
        i += 1;
    }
    s
}

fn fvla(m: usize, c: &Vec<Vec<i32>>) {
    let mut d = vec![0i32; m];
    let mut i = 0usize;
    while i < m {
        d[i] = ((i + 1) as i32).wrapping_mul(7).wrapping_add(3);
        i += 1;
    }

    let mut q = backing().lock().unwrap();
    i = 0;
    while i < m {
        let mut j = 0usize;
        while j < m {
            let idx = (i + j) % m;
            q[i][j] = c[i][j].wrapping_add(d[idx]);
            j += 1;
        }
        i += 1;
    }

    let checksum = checksum_2d(m, &q);
    drop(q);
    SINK.fetch_xor(checksum, Ordering::SeqCst);
}

fn main() {
    let m = N as usize;

    let mut x = vec![vec![0i32; m]; m];
    let mut y = vec![vec![0i32; m]; m];

    fill_backing();

    {
        let mut i = 0usize;
        while i < m {
            let mut j = 0usize;
            while j < m {
                x[i][j] = ((i + 1) as i32).wrapping_mul(10).wrapping_add((j + 1) as i32);
                y[i][j] = ((i + 1) as i32).wrapping_mul(20).wrapping_add((j + 1) as i32);
                j += 1;
            }
            i += 1;
        }
    }

    fvla(m, &x);

    {
        let mut expect = vec![vec![0i32; m]; m];
        let mut i = 0usize;
        while i < m {
            let mut j = 0usize;
            while j < m {
                let d = ((i + j) % m + 1) as i32;
                let d = d.wrapping_mul(7).wrapping_add(3);
                expect[i][j] = x[i][j].wrapping_add(d);
                j += 1;
            }
            i += 1;
        }
        let backing_checksum = {
            let b = backing().lock().unwrap();
            checksum_2d(m, &b)
        };
        if backing_checksum != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = vec![vec![0i32; m]; m];
        let mut i = 0usize;
        while i < m {
            let mut j = 0usize;
            while j < m {
                let d = ((i + j) % m + 1) as i32;
                let d = d.wrapping_mul(7).wrapping_add(3);
                expect2[i][j] = y[i][j].wrapping_add(d);
                j += 1;
            }
            i += 1;
        }
        let backing_checksum = {
            let b = backing().lock().unwrap();
            checksum_2d(m, &b)
        };
        if backing_checksum != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    if SINK.load(Ordering::SeqCst) == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}