use std::cell::RefCell;

lazy_static::lazy_static! {
    static ref BACKING: RefCell<[[i32; 9]; 9]> = RefCell::new([[0; 9]; 9]);
    static ref SINK: RefCell<i32> = RefCell::new(0);
}

fn fill_backing(backing: &mut [[i32; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = ((i + 1) as i32) * 100 + (j + 1) as i32;
        }
    }
}

fn checksum_2d(m: usize, a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..m {
        for j in 0..m {
            s ^= a[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(m: usize, c: &mut [[i32; 9]; 9]) {
    let mut d = [0i32; 9];
    for i in 0..m {
        d[i] = ((i + 1) as i32) * 7 + 3;
    }
    let mut q = BACKING.borrow_mut();
    for i in 0..m {
        for j in 0..m {
            q[i][j] = c[i][j] + d[((i + j) % m) as usize];
        }
    }
    let chk = checksum_2d(m, &*q);
    *SINK.borrow_mut() ^= chk;
}

fn main() {
    const N: i32 = 9;
    let m = N as usize;

    fill_backing(&mut *BACKING.borrow_mut());

    let mut x = [[0i32; 9]; 9];
    let mut y = [[0i32; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = ((i + 1) as i32) * 10 + (j + 1) as i32;
            y[i][j] = ((i + 1) as i32) * 20 + (j + 1) as i32;
        }
    }

    fvla(m, &mut x);

    {
        let mut expect = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(m, &*BACKING.borrow()) != checksum_2d(m, &expect) {
            std::process::exit(1);
        }
    }

    fvla(m, &mut y);

    {
        let mut expect2 = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d = (((i + j) % m + 1) as i32) * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(m, &*BACKING.borrow()) != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }

    if *SINK.borrow() == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}