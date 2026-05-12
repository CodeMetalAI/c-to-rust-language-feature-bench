use std::sync::{Mutex, OnceLock};

static N: i32 = 9;

fn backing() -> &'static Mutex<[[i32; 9]; 9]> {
    static BACKING: OnceLock<Mutex<[[i32; 9]; 9]>> = OnceLock::new();
    BACKING.get_or_init(|| Mutex::new([[0; 9]; 9]))
}

fn sink() -> &'static Mutex<i32> {
    static SINK: OnceLock<Mutex<i32>> = OnceLock::new();
    SINK.get_or_init(|| Mutex::new(0))
}

fn fill_backing() {
    let mut b = backing().lock().unwrap();
    for i in 0..9usize {
        for j in 0..9usize {
            b[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0i32;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9]) {
    let mut d = vec![0i32; m];
    for i in 0..m {
        d[i] = (i as i32 + 1) * 7 + 3;
    }

    let checksum;
    {
        let mut q = backing().lock().unwrap();
        for i in 0..m {
            for j in 0..m {
                q[i][j] = c[i][j] + d[(i + j) % m];
            }
        }
        checksum = checksum_2d(m, &q);
    }

    let mut s = sink().lock().unwrap();
    *s ^= checksum;
}

fn main() {
    let m = N as usize;

    let mut x = [[0i32; 9]; 9];
    let mut y = [[0i32; 9]; 9];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(m, &x);

    {
        let mut expect = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        let b = backing().lock().unwrap();
        if checksum_2d(m, &b) != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &y);

    {
        let mut expect2 = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        let b = backing().lock().unwrap();
        if checksum_2d(m, &b) != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    let s = sink().lock().unwrap();
    if *s == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}