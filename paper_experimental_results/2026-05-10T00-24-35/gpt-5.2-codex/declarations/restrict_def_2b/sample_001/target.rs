use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, mut p: i32, mut q: i32, arr: &mut [i32]) {
    let mut n = n;
    while n > 0 {
        let val = arr[q as usize];
        arr[p as usize] = val;
        p += 1;
        q += 1;
        n -= 1;
    }
}

fn safe_move(n: i32, p: i32, q: i32, arr: &mut [i32]) {
    if n <= 0 {
        return;
    }
    if p < q {
        let mut i = 0;
        while i < n {
            let val = arr[(q + i) as usize];
            arr[(p + i) as usize] = val;
            i += 1;
        }
    } else if p > q {
        let mut i = n;
        while i > 0 {
            i -= 1;
            let val = arr[(q + i) as usize];
            arr[(p + i) as usize] = val;
        }
    } else {
        return;
    }
}

fn ranges_overlap(n_total: i32, p: i32, q: i32, n: i32) -> bool {
    let g = GATE.load(Ordering::Relaxed);
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

fn call_f_checked(n_total: i32, n: i32, p: i32, q: i32, arr: &mut [i32]) {
    if ranges_overlap(n_total, p, q, n) {
        safe_move(n, p, q, arr);
    } else {
        f(n, p, q, arr);
    }
}

fn init(arr: &mut [i32], n: i32) {
    let mut i = 0;
    while i < n {
        arr[i as usize] = i * 13 + 5;
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

fn g_defined(arr: &mut [i32]) {
    call_f_checked(100, 50, 50, 0, arr);
    call_f_checked(100, 50, 1, 0, arr);
}

fn main() {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, 50, 0, &mut expect);
    safe_move(50, 1, 0, &mut expect);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        exit(1);
    }
}