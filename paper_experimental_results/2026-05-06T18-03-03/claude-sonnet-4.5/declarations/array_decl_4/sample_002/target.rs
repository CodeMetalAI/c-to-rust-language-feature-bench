use std::sync::atomic::{AtomicI32, Ordering};

static N: i32 = 9;
static SINK: AtomicI32 = AtomicI32::new(0);
static mut BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            unsafe {
                BACKING[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
            }
            j += 1;
        }
        i += 1;
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
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

fn fvla(m: usize, c: &[[i32; 9]; 9]) {
    let mut d = [0i32; 9];
    
    let mut i = 0;
    while i < m {
        d[i] = (i as i32 + 1) * 7 + 3;
        i += 1;
    }
    
    i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            unsafe {
                BACKING[i][j] = c[i][j] + d[(i + j) % m];
            }
            j += 1;
        }
        i += 1;
    }
    
    let checksum = unsafe { checksum_2d(m, &BACKING) };
    SINK.fetch_xor(checksum, Ordering::SeqCst);
}

fn main() {
    let m = N as usize;
    
    let mut x = [[0i32; 9]; 9];
    let mut y = [[0i32; 9]; 9];
    
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
        let mut expect = [[0i32; 9]; 9];
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
        if unsafe { checksum_2d(m, &BACKING) } != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }
    
    fvla(m, &y);
    
    {
        let mut expect2 = [[0i32; 9]; 9];
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
        if unsafe { checksum_2d(m, &BACKING) } != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }
    
    if SINK.load(Ordering::SeqCst) == 0 {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}