use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static N: RefCell<i32> = RefCell::new(0);
    static M: RefCell<i32> = RefCell::new(0);
    static GATE: RefCell<i32> = RefCell::new(0);
}

fn check_a(nn: i32, mm: i32, a: &Vec<Vec<Vec<i32>>>, mut p_idx: usize) -> i32 {
    let mut i = 0;
    while i < nn {
        let p_slice = &a[p_idx];
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i as usize][j as usize][k as usize] != p_slice[j as usize][k as usize] + i {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_idx += 1;
    }
    0
}

fn check_c(nn: i32, mm: i32, c: &Vec<Vec<Vec<Vec<i32>>>>, r: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i as usize][j as usize][u as usize][v as usize] != r[i as usize][j as usize][u as usize][v as usize] {
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
    let nn = N.with(|n| *n.borrow());
    let mm = M.with(|m| *m.borrow());
    let mut a: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; mm as usize]; 6]; nn as usize];
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i as usize][j as usize][k as usize] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    let mut c: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0; mm as usize]; 6]; nn as usize]; nn as usize];
    i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    c[i as usize][j as usize][u as usize][v as usize] = (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }
    if check_a(nn, mm, &a, 0) != 0 {
        GATE.with(|g| *g.borrow_mut() = 1);
    }
    if check_c(nn, mm, &c, &c) != 0 {
        GATE.with(|g| *g.borrow_mut() = 2);
    }
}

fn main() {
    N.with(|n| *n.borrow_mut() = 6);
    M.with(|m| *m.borrow_mut() = N.with(|n| *n.borrow()) + 1);
    GATE.with(|g| *g.borrow_mut() = 0);
    fcompat();
    if GATE.with(|g| *g.borrow()) != 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}