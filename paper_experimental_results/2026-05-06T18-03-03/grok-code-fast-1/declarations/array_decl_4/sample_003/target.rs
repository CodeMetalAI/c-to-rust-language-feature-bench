use std::sync::Mutex;

static BACKING: Mutex<[[i32; 9]; 9]> = Mutex::new([[0; 9]; 9]);
static SINK: Mutex<i32> = Mutex::new(0);

fn fill_backing() {
    let mut backing = BACKING.lock().unwrap();
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
        }
    }
}

fn checksum_2d(arr: &[[i32; 9]; 9]) -> i32 {
    let mut s = 0;
    for i in 0..9 {
        for j in 0..9 {
            s ^= arr[i][j] + (i as i32) * 131 + (j as i32) * 17;
        }
    }
    s
}

fn fvla(c: &[[i32; 9]; 9]) {
    let mut d = vec![0i32; 9];
    for i in 0..9 {
        d[i] = ((i as i32) + 1) * 7 + 3;
    }

    let mut backing = BACKING.lock().unwrap();
    for i in 0..9 {
        for j in 0..9 {
            backing[i][j] = c[i][j] + d[((i + j) % 9) as usize];
        }
    }

    let checksum = checksum_2d(&*backing);
    *SINK.lock().unwrap() ^= checksum;
}

fn main() {
    let m = 9;

    let mut x = [[0i32; 9]; 9];
    let mut y = [[0i32; 9]; 9];

    fill_backing();

    for i in 0..m {
        for j in 0..m {
            x[i][j] = ((i as i32) + 1) * 10 + (j as i32 + 1);
            y[i][j] = ((i as i32) + 1) * 20 + (j as i32 + 1);
        }
    }

    fvla(&x);

    {
        let mut expect = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d_val = (((i + j) % m) as i32 + 1) * 7 + 3;
                expect[i][j] = x[i][j] + d_val;
            }
        }
        let backing_checksum = checksum_2d(&*BACKING.lock().unwrap());
        let expect_checksum = checksum_2d(&expect);
        if backing_checksum != expect_checksum {
            std::process::exit(1);
        }
    }

    fvla(&y);

    {
        let mut expect2 = [[0i32; 9]; 9];
        for i in 0..m {
            for j in 0..m {
                let d_val = (((i + j) % m) as i32 + 1) * 7 + 3;
                expect2[i][j] = y[i][j] + d_val;
            }
        }
        let backing_checksum = checksum_2d(&*BACKING.lock().unwrap());
        let expect2_checksum = checksum_2d(&expect2);
        if backing_checksum != expect2_checksum {
            std::process::exit(2);
        }
    }

    if *SINK.lock().unwrap() == 0 {
        std::process::exit(3);
    }

    std::process::exit(0);
}