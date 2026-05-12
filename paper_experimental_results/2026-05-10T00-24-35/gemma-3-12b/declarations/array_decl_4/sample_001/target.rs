#[allow(dead_code)]
static mut SINK: i32 = 0;

#[allow(dead_code)]
static BACKING: [[i32; 9]; 9] = [[0; 9]; 9];

#[allow(dead_code)]
static mut N: i32 = 9;

#[inline(always)]
fn fill_backing() {
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            unsafe {
                BACKING[i][j] = (i + 1) * 100 + (j + 1);
            }
            j += 1;
        }
        i += 1;
    }
}

#[inline(always)]
fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            s ^= a[i][j] + i * 131 + j * 17;
            j += 1;
        }
        i += 1;
    }
    s
}

fn fvla(m: usize, c: &[[i32; 9]; 9]) {
    let mut d = vec![0; m];
    let s = c;
    let mut q = &mut BACKING;

    let mut i = 0;
    while i < m {
        d[i] = (i + 1) * 7 + 3;
        i += 1;
    }

    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            q[i][j] = s[i][j] + d[(i + j) % m];
            j += 1;
        }
        i += 1;
    }

    unsafe {
        SINK ^= checksum_2d(m, &BACKING);
    }
}

fn main() {
    let m = unsafe { N } as usize;

    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    fill_backing();

    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
            j += 1;
        }
        i += 1;
    }

    fvla(m, &x);

    let mut expect = [[0; 9]; 9];
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
            j += 1;
        }
        i += 1;
    }

    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect) {
        return 1;
    }

    fvla(m, &y);

    let mut expect2 = [[0; 9]; 9];
    let mut i = 0;
    while i < m {
        let mut j = 0;
        while j < m {
            let d = ((i + j) % m + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
            j += 1;
        }
        i += 1;
    }

    if checksum_2d(m, &BACKING) != checksum_2d(m, &expect2) {
        return 2;
    }

    unsafe {
        if SINK == 0 {
            return 3;
        }
    }

    0
}