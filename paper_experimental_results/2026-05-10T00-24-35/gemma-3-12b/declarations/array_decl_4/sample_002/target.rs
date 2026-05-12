fn main() {
    let n = 9;
    volatile static mut sink: i32 = 0;
    static mut backing: [[i32; 9]; 9] = [[0; 9]; 9];

    unsafe {
        fill_backing();

        let m = n;

        let mut x: [[i32; 9]; 9] = [[0; 9]; 9];
        let mut y: [[i32; 9]; 9] = [[0; 9]; 9];

        {
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
        }

        fvla(m, &mut x);

        {
            let mut expect: [[i32; 9]; 9] = [[0; 9]; 9];
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
            if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
                return 1;
            }
        }

        fvla(m, &mut y);

        {
            let mut expect2: [[i32; 9]; 9] = [[0; 9]; 9];
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
            if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
                return 2;
            }
        }

        if sink == 0 {
            return 3;
        }
    }

    0
}

static volatile mut sink: i32 = 0;
static mut backing: [[i32; 9]; 9] = [[0; 9]; 9];

fn fill_backing() {
    unsafe {
        let mut i = 0;
        while i < 9 {
            let mut j = 0;
            while j < 9 {
                backing[i][j] = (i + 1) * 100 + (j + 1);
                j += 1;
            }
            i += 1;
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    unsafe {
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
}

fn fvla(m: usize, c: &mut [[i32; 9]; 9]) {
    unsafe {
        let mut d: [i32; 9] = [0; 9];
        let s = c;
        let mut q = &mut backing;

        let mut i = 0;
        while i < m {
            d[i] = (i + 1) * 7 + 3;
            i += 1;
        }

        i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                q[i][j] = s[i][j] + d[(i + j) % m];
                j += 1;
            }
            i += 1;
        }

        sink ^= checksum_2d(m, q);
    }
}