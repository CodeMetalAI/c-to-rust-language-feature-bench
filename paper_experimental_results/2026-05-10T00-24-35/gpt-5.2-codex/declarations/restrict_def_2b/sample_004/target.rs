use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, data: &mut [i32], mut p: usize, mut q: usize) {
    let mut count = n;
    while count > 0 {
        data[p] = data[q];
        p += 1;
        q += 1;
        count -= 1;
    }
}

fn safe_move(n: i32, data: &mut [i32], p: usize, q: usize) {
    if n <= 0 {
        return;
    }
    let n_usize = n as usize;
    if p < q {
        let mut i = 0usize;
        while i < n_usize {
            data[p + i] = data[q + i];
            i += 1;
        }
    } else if p > q {
        let mut i = n_usize;
        while i > 0 {
            i -= 1;
            data[p + i] = data[q + i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(n_total: i32, p: i32, q: i32, n: i32) -> bool {
    let g = GATE.load(Ordering::SeqCst);
    let mut ps = p;
    let mut qs = q;

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

fn call_f_checked(data: &mut [i32], n_total: i32, n: i32, p: usize, q: usize) {
    if ranges_overlap(n_total, p as i32, q as i32, n) {
        safe_move(n, data, p, q);
    } else {
        f(n, data, p, q);
    }
}

fn init(x: &mut [i32], n: i32) {
    let mut i = 0;
    while i < n {
        x[i as usize] = i * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    let mut i = 0;
    while i < n {
        if x[i as usize] != y[i as usize] {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect, 50, 0);
    safe_move(50, &mut expect, 1, 0);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        exit(1);
    }
}