use std::cell::Cell;

thread_local! {
    static SINK: Cell<i32> = Cell::new(0);
    static BACKING: Cell<[[i32; 9]; 9]> = Cell::new([[0; 9]; 9]);
}

fn fill_backing() {
    BACKING.with(|backing| {
        let mut arr = backing.get();
        let mut i = 0;
        while i < 9 {
            let mut j = 0;
            while j < 9 {
                arr[i][j] = (i as i32 + 1) * 100 + (j as i32 + 1);
                j += 1;
            }
            i += 1;
        }
        backing.set(arr);
    });
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
    
    BACKING.with(|backing| {
        let mut q = backing.get();
        
        i = 0;
        while i < m {
            let mut j = 0;
            while j < m {
                q[i][j] = c[i][j] + d[(i + j) % m];
                j += 1;
            }
            i += 1;
        }
        
        backing.set(q);
        
        SINK.with(|sink| {
            sink.set(sink.get() ^ checksum_2d(m, &q));
        });
    });
}

fn main() {
    let n: usize = 9;
    let m = n;
    
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
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect[i][j] = x[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing_val = BACKING.with(|b| b.get());
        if checksum_2d(m, &backing_val) != checksum_2d(m, &expect) {
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
                let d = ((i + j) % m + 1) as i32 * 7 + 3;
                expect2[i][j] = y[i][j] + d;
                j += 1;
            }
            i += 1;
        }
        let backing_val = BACKING.with(|b| b.get());
        if checksum_2d(m, &backing_val) != checksum_2d(m, &expect2) {
            std::process::exit(2);
        }
    }
    
    let sink_val = SINK.with(|s| s.get());
    if sink_val == 0 {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}