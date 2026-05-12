use std::cell::Cell;

thread_local! {
    static N: Cell<i32> = Cell::new(0);
    static M: Cell<i32> = Cell::new(0);
    static GATE: Cell<i32> = Cell::new(0);
}

fn check_a(nn: usize, mm: usize, a: &Vec<Vec<Vec<i32>>>, p: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i][j][k] != p[i][j][k] {
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

fn check_c(nn: usize, _mm: usize, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < r[0][0][0].len() {
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
    let nn = N.with(|n| n.get()) as usize;
    let mm = M.with(|m| m.get()) as usize;

    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm]; 6]; nn];

    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm]; 6]; nn]; nn];

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i][j][k] = ((i + 1) * 10000 + (j + 1) * 100 + (k + 1)) as i32;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    c[i][j][u][v] =
                        ((i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1)) as i32;
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    let p = &a;
    if check_a(nn, mm, &a, p) != 0 {
        GATE.with(|g| g.set(1));
    }

    let r = &c;
    if check_c(nn, mm, &c, r) != 0 {
        GATE.with(|g| g.set(2));
    }
}

fn main() {
    N.with(|n| n.set(6));
    M.with(|m| m.set(N.with(|n| n.get()) + 1));

    GATE.with(|g| g.set(0));
    fcompat();

    if GATE.with(|g| g.get()) != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}