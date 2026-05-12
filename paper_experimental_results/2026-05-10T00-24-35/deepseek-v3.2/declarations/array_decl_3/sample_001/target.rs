static mut N: i32 = 0;
static mut M: i32 = 0;
static mut GATE: i32 = 0;

fn check_a(nn: usize, mm: usize, a: &[Vec<[i32; 6]>], p: &[[i32; 6]]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[j][k] + i as i32 {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn check_c(nn: usize, mm: usize, c: &[Vec<Vec<[i32; 6]>>], r: &[Vec<Vec<[i32; 6]>>]) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i][j][u][v] != r[i][j][u][v] {
                        return 1;
                    }
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

fn fcompat() {
    let nn = unsafe { N as usize };
    let mm = unsafe { M as usize };

    let mut a = Vec::with_capacity(nn);
    for i in 0..nn {
        let mut inner = Vec::with_capacity(6);
        for j in 0..6 {
            let mut row = [0; 6];
            for k in 0..mm {
                row[k] = (i as i32 + 1) * 10000 + (j as i32 + 1) * 100 + (k as i32 + 1);
            }
            inner.push(row);
        }
        a.push(inner);
    }

    let mut c = Vec::with_capacity(nn);
    for i in 0..nn {
        let mut outer = Vec::with_capacity(nn);
        for j in 0..nn {
            let mut middle = Vec::with_capacity(6);
            for u in 0..6 {
                let mut row = [0; 6];
                for v in 0..mm {
                    row[v] = (i as i32 + 1) * 1000000 + (j as i32 + 1) * 10000 + (u as i32 + 1) * 100 + (v as i32 + 1);
                }
                middle.push(row);
            }
            outer.push(middle);
        }
        c.push(outer);
    }

    let p_flat: Vec<[i32; 6]> = a.iter().flat_map(|inner| inner.iter().cloned()).collect();
    let p_chunks: Vec<&[[i32; 6]]> = p_flat.chunks(6).collect();
    if check_a(nn, mm, &a, &p_chunks[0..6]) != 0 {
        unsafe { GATE = 1; }
    }

    let r_flat: Vec<Vec<[i32; 6]>> = c.clone();
    if check_c(nn, mm, &c, &r_flat) != 0 {
        unsafe { GATE = 2; }
    }
}

fn main() {
    unsafe {
        N = 6;
        M = N + 1;
        GATE = 0;
    }

    fcompat();

    unsafe {
        if GATE != 0 {
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}