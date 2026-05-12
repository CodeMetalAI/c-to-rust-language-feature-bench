use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

static N: i32 = 9;
static SINK: AtomicI32 = AtomicI32::new(0);
static BACKING: Mutex<[[i32; 9]; 9]> = Mutex::new([[0; 9]; 9]);

fn fill_backing() {
    let mut backing = BACKING.lock().unwrap();
    let mut i = 0usize;
    while i < 9 {
        let mut j = 0usize;
        while j < 9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s: i32 = 0;
    let mut i = 0usize;
    while i < m {
        let mut j = 0usize;
        while j < m {
            let ii = i as i32;
            let jj = j as i32;
            s ^= a[i][j] + ii * 131 + jj * 17;
            j += 1;
        }
        i += 1;
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9]) {
    let mut d: Vec<i32> = vec![0; m];
    let mut i = 0usize;
    while i < m {
        d[i] = (i as i32 + 1) * 7 + 3;
        i += 1;
    }

    let mut backing = BACKING.lock().unwrap();
    i = 0;
    while i < m {
        let mut j = 0usize;
        while j < m {
            backing[i][j] = c[i][j] + d[(i + j) % m];
            j += 1;
        }
        i += 1;
    }

    let cs = checksum_2d(m, &*backing);
    SINK.fetch_xor(cs, Ordering::SeqCst);
}

fn main() {
    let m = N as usize;

    let mut x: [[i32; 9]; 9] = [[0; 9]; 9];
    let mut y: [[i32; 9]; 9] = [[0; 9]; 9];

    fill_backing();

    {
        let mut i = 0usize;
        while i < m {
            let mut j = 0usize;
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
        let mut expect: [[i32; 9]; 9] = [[0; 9]; 9];
        let mut i = 0usize;
        while i < m {
            let mut j = 0usize;
            while j < m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing = BACKING.lock().unwrap();
        if checksum_2d(m, &*backing) != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2: [[i32; 9]; 9] = [[0; 9]; 9];
        let mut i = 0usize;
        while i < m {
            let mut j = 0usize;
            while j < m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing = BACKING.lock().unwrap();
        if checksum_2d(m, &*backing) != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    if SINK.load(Ordering::SeqCst) == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}