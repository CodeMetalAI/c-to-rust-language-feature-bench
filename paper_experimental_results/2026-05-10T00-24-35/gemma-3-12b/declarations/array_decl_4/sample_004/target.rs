fn main() {
    let n = 9;
    static mut sink: i32 = 0;

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

    fn checksum_2d(m: i32, a: &[[i32; 9]; 9]) -> i32 {
        unsafe {
            let mut s = 0;
            let mut i = 0;
            while i < m {
                let mut j = 0;
                while j < m {
                    s ^= a[i as usize][j as usize] + (i * 131) + (j * 17);
                    j += 1;
                }
                i += 1;
            }
            s
        }
    }

    fn fvla(m: i32, c: &[[i32; 9]; 9]) {
        unsafe {
            let mut d = vec![0; m as usize];
            let s = c;
            let mut q = &mut backing;

            let mut i = 0;
            while i < m {
                d[i as usize] = (i + 1) * 7 + 3;
                i += 1;
            }

            let mut i = 0;
            while i < m {
                let mut j = 0;
                while j < m {
                    q[i as usize][j as usize] = s[i as usize][j as usize] + d[( (i + j) % m) as usize];
                    j += 1;
                }
                i += 1;
            }

            sink ^= checksum_2d(m, &backing);
        }
    }

    let m = n;

    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];

    unsafe {
        fill_backing();
    }

    unsafe {
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                x[i as usize][j as usize] = (i + 1) * 10 + (j + 1);
                y[i as usize][j as usize] = (i + 1) * 20 + (j + 1);
                j += 1;
            }
            i += 1;
        }
    }

    unsafe {
        fvla(m, &x);
    }

    unsafe {
        let mut expect = [[0; 9]; 9];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = ((i + j) % m + 1) * 7 + 3;
                expect[i as usize][j as usize] = x[i as usize][j as usize] + d;
                j += 1;
            }
            i += 1;
        }
        if checksum_2d(m, &backing) != checksum_2d(m, &expect) {
            return 1;
        }
    }

    unsafe {
        fvla(m, &y);
    }

    unsafe {
        let mut expect2 = [[0; 9]; 9];
        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                let d = ((i + j) % m + 1) * 7 + 3;
                expect2[i as usize][j as usize] = y[i as usize][j as usize] + d;
                j += 1;
            }
            i += 1;
        }
        if checksum_2d(m, &backing) != checksum_2d(m, &expect2) {
            return 2;
        }
    }

    unsafe {
        if sink == 0 {
            return 3;
        }
    }

    0
}