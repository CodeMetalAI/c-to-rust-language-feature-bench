use std::sync::Mutex;

static N: Mutex<i32> = Mutex::new(0);
static M: Mutex<i32> = Mutex::new(0);
static GATE: Mutex<i32> = Mutex::new(0);

fn set_n(v: i32) {
    *N.lock().unwrap() = v;
}

fn get_n() -> i32 {
    *N.lock().unwrap()
}

fn set_m(v: i32) {
    *M.lock().unwrap() = v;
}

fn get_m() -> i32 {
    *M.lock().unwrap()
}

fn set_gate(v: i32) {
    *GATE.lock().unwrap() = v;
}

fn get_gate() -> i32 {
    *GATE.lock().unwrap()
}

fn check_a(nn: i32, mm: i32, a: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut i = 0;
    let mut p_index = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                if a[i as usize][j as usize][k as usize]
                    != a[p_index as usize][j as usize][k as usize] + i
                {
                    return 1;
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
        p_index += 1;
    }
    0
}

fn check_c(nn: i32, mm: i32, c: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < nn {
            let mut u = 0;
            while u < 6 {
                let mut v = 0;
                while v < mm {
                    if c[i as usize][j as usize][u as usize][v as usize]
                        != c[i as usize][j as usize][u as usize][v as usize]
                    {
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
    let nn = get_n();
    let mm = get_m();

    let mut a = vec![vec![vec![0i32; mm as usize]; 6]; nn as usize];
    let mut c = vec![vec![vec![vec![0i32; mm as usize]; 6]; nn as usize]; nn as usize];

    let mut i = 0;
    while i < nn {
        let mut j = 0;
        while j < 6 {
            let mut k = 0;
            while k < mm {
                a[i as usize][j as usize][k as usize] =
                    (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
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
                    c[i as usize][j as usize][u as usize][v as usize] =
                        (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1);
                    v += 1;
                }
                u += 1;
            }
            j += 1;
        }
        i += 1;
    }

    if check_a(nn, mm, &a) != 0 {
        set_gate(1);
    }

    if check_c(nn, mm, &c) != 0 {
        set_gate(2);
    }
}

fn main() {
    set_n(6);
    let n = get_n();
    set_m(n + 1);

    set_gate(0);
    fcompat();

    if get_gate() != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}