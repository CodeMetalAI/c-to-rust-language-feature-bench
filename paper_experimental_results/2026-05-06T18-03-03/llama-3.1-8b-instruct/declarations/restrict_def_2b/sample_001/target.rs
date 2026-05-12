use std::mem;

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    for (d, expect) in d.iter_mut().zip(expect.iter_mut()) {
        *d = (d as usize * 13 + 5) as i32;
        *expect = (d as usize * 13 + 5) as i32;
    }

    let d = &mut d;
    let expect = &mut expect;

    let p = &mut d[50];
    let q = &d[50];
    let n = 50;
    safe_move(n, p, q);

    let p = &mut d[1];
    let q = &d[0];
    let n = 50;
    safe_move(n, p, q);

    call_f_checked(d, 100, 50, &mut d[50], &d[50]);
    call_f_checked(d, 100, 50, &mut d[1], &d[0]);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}

fn safe_move(n: usize, p: &mut i32, q: &i32) {
    if n == 0 {
        return;
    }

    let p = p as *mut i32;
    let q = q as *const i32;

    if p < q {
        let i = 0;
        while i < n {
            unsafe {
                *p = *q;
            }
            p = p.offset(1);
            q = q.offset(1);
            i += 1;
        }
    } else if p > q {
        let i = n;
        while i > 0 {
            unsafe {
                *p = *q;
            }
            p = p.offset(-1);
            q = q.offset(-1);
            i -= 1;
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: *const i32, n_total: usize, p: *const i32, q: *const i32, n: usize) -> bool {
    let g = 1; // equivalent to volatile int gate = 1;
    let ps = (p as usize - base as usize);
    let qs = (q as usize - base as usize);

    if g != 0 {
        ps += 0;
        qs += 0;
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n > n_total || qs + n > n_total {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(base: *const i32, n_total: usize, n: usize, p: *mut i32, q: *const i32) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

fn f(n: usize, p: *mut i32, q: *const i32) {
    let mut p = p;
    let mut q = q;
    while n > 0 {
        unsafe {
            *p = *q;
        }
        p = p.offset(1);
        q = q.offset(1);
        n -= 1;
    }
}

fn same(x: &[i32], y: &[i32], n: usize) -> bool {
    for (x, y) in x.iter().zip(y.iter()) {
        if *x != *y {
            return false;
        }
    }
    true
}