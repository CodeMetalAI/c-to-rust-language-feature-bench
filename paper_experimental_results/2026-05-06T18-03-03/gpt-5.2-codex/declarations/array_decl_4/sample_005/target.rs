use std::sync::{Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

const N: usize = 9;

static BACKING: Mutex<[[i32; N]; N]> = Mutex::new([[0; N]; N]);
static SINK: AtomicI32 = AtomicI32::new(0);

fn fill_backing() {
    let mut backing = BACKING.lock().unwrap();
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            backing[i][j] = ((i + 1) as i32) * 100 + (j + 1) as i32;
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &[[i32; N]; N]) -> i32 {
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

fn fvla(m: usize, c: &[[i32; N]; N]) {
    let mut d = vec![0i32; m];
    let mut i = 0;
    while i < m {
        d[i] = ((i + 1) as i32) * 7 + 3;
        i += 1;
    }

    {
        let mut backing = BACKING.lock().unwrap();
        i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                backing[i][j] = c[i][j] + d[(i + j) % m];
                j += 1;
            }
            i += 1;
        }
        let cs = checksum_2d(m, &backing);
        SINK.fetch_xor(cs, Ordering::SeqCst);
    }
}

fn main() {
    let m = N;

    let mut x = [[0i32; N]; N];
    let mut y = [[0i32; N]; N];

    fill_backing();

    {
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                x[i][j] = ((i + 1) as i32) * 10 + (j + 1) as i32;
                y[i][j] = ((i + 1) as i32) * 20 + (j + 1) as i32;
                j += 1;
            }
            i += 1;
        }
    }

    fvla(m, &x);

    {
        let mut expect = [[0i32; N]; N];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect[i][j] = x[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing = BACKING.lock().unwrap();
        if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
            exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = [[0i32; N]; N];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing = BACKING.lock().unwrap();
        if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
            exit(2);
        }
    }

    if SINK.load(Ordering::SeqCst) == 0 {
        exit(3);
    }

    exit(0);
}