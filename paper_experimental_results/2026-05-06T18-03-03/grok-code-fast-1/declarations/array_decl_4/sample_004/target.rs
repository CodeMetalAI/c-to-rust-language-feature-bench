use std::cell::RefCell;

thread_local! {
    static BACKING: RefCell<[[i32; 9]; 9]> = RefCell::new([[0; 9]; 9]);
    static SINK: RefCell<i32> = RefCell::new(0);
}

fn fill_backing() {
    BACKING.with(|b| {
        let mut backing = b.borrow_mut();
        for i in 0..9 {
            for j in 0..9 {
                backing[i][j] = (i + 1) * 100 + (j + 1);
            }
        }
    });
}

fn checksum_2d(a: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..9 {
        for j in 0..9 {
            s ^= a[i][j] + i as i32 * 131 + j as i32 * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; 9]; 9]) {
    let mut d = [0; 9];
    for i in 0..9 {
        d[i] = (i + 1) * 7 + 3;
    }
    BACKING.with(|b| {
        let mut q = b.borrow_mut();
        for i in 0..9 {
            for j in 0..9 {
                q[i][j] = c[i][j] + d[(i + j) % 9];
            }
        }
        let checksum = checksum_2d(&*q);
        SINK.with(|s| {
            *s.borrow_mut() ^= checksum;
        });
    });
}

fn main() -> i32 {
    let mut x = [[0; 9]; 9];
    let mut y = [[0; 9]; 9];
    fill_backing();
    for i in 0..9 {
        for j in 0..9 {
            x[i][j] = (i + 1) * 10 + (j + 1);
            y[i][j] = (i + 1) * 20 + (j + 1);
        }
    }
    fvla(&x);
    let mut expect = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = ((i + j) % 9 + 1) * 7 + 3;
            expect[i][j] = x[i][j] + d;
        }
    }
    let expect_checksum = checksum_2d(&expect);
    let backing_checksum = BACKING.with(|b| checksum_2d(&*b.borrow()));
    if backing_checksum != expect_checksum {
        return 1;
    }
    fvla(&y);
    let mut expect2 = [[0; 9]; 9];
    for i in 0..9 {
        for j in 0..9 {
            let d = ((i + j) % 9 + 1) * 7 + 3;
            expect2[i][j] = y[i][j] + d;
        }
    }
    let expect2_checksum = checksum_2d(&expect2);
    let backing_checksum2 = BACKING.with(|b| checksum_2d(&*b.borrow()));
    if backing_checksum2 != expect2_checksum {
        return 2;
    }
    let sink_val = SINK.with(|s| *s.borrow());
    if sink_val == 0 {
        return 3;
    }
    0
}