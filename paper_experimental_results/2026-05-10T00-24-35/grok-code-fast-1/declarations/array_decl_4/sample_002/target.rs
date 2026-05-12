use std::cell::RefCell;

lazy_static::lazy_static! {
    static ref BACKING: RefCell<[[i32; 9]; 9]> = RefCell::new([[0; 9]; 9]);
    static ref SINK: RefCell<i32> = RefCell::new(0);
}

fn fill_backing() {
    let mut backing_mut = BACKING.borrow_mut();
    for i in 0..9 {
        for j in 0..9 {
            backing_mut[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0i32;
    for i in 0..9 {
        for j in 0..9 {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; 9]; 9]) {
    let mut d = vec![0i32; 9];
    for i in 0..9 {
        d[i] = (i as i32 + 1) * 7 + 3;
    }
    let mut backing_mut = BACKING.borrow_mut();
    for i in 0..9 {
        for j in 0..9 {
            backing_mut[i][j] = c[i][j] + d[(i + j) % 9];
        }
    }
    let checksum = checksum_2d(&*backing_mut);
    *SINK.borrow_mut() ^= checksum;
}

fn main() {
    let mut x: [[i32; 9]; 9] = [[0; 9]; 9];
    let mut y: [[i32; 9]; 9] = [[0; 9]; 9];

    fill_backing();

    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = (i as i32 + 1) * 10 + (j as i32 + 1);
            y[i][j] = (i as i32 + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(&x);

    {
        let mut expect: [[i32; 9]; 9] = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let d = ((i + j) % 9 + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
            }
        }
        if checksum_2d(&*BACKING.borrow()) != checksum_2d(&expect) {
            std::process::exit(1);
        }
    }

    fvla(&y);

    {
        let mut expect2: [[i32; 9]; 9] = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                let d = ((i + j) % 9 + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
            }
        }
        if checksum_2d(&*BACKING.borrow()) != checksum_2d(&expect2) {
            std::process::exit(2);
        }
    }

    if *SINK.borrow() == 0 {
        std::process::exit(3);
    }
}