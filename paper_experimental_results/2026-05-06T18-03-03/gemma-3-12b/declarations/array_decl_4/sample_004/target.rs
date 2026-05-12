fn main() {
    let n = 9;
    volatile static mut sink: i32 = 0;
    static mut backing: [[i32; 9]; 9] = [[0; 9]; 9];

    unsafe {
        fn fill_backing() {
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
            let mut q: &[[i32; 9]; 9] = &backing;

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

        fill_backing();

        let mut x = [[0; 9]; 9];
        let mut y = [[0; 9]; 9];

        let mut i = 0;
        while i < 9 {
            let mut j = 0;
            while j < 9 {
                x[i][j] = (i + 1) * 10 + (j + 1);
                y[i][j] = (i + 1) * 20 + (j + 1);
                j += 1;
            }
            i += 1;
        }

        fvla(9, &x);

        let mut expect = [[0; 9]; 9];
        i = 0;
        while i < 9 {
            let mut j = 0;
            while j < 9 {
                let d = ((i + j) % 9 + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        if checksum_2d(9, &backing) != checksum_2d(9, &expect) {
            std::process::exit(1);
        }

        fvla(9, &y);

        let mut expect2 = [[0; 9]; 9];
        i = 0;
        while i < 9 {
            let mut j = 0;
            while j < 9 {
                let d = ((i + j) % 9 + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        if checksum_2d(9, &backing) != checksum_2d(9, &expect2) {
            std::process::exit(2);
        }

        if sink == 0 {
            std::process::exit(3);
        }

        std::process::exit(0);
    }
}