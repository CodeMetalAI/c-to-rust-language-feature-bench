use std::sync::atomic::{AtomicI32, Ordering};
use std::process::exit;

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, base: &mut [i32], mut p: i32, mut q: i32) {
    let mut nn = n;
    while nn > 0 {
        let val = base[q as usize];
        base[p as usize] = val;
        p += 1;
        q += 1;
        nn -= 1;
    }
}

fn safe_move(n: i32, base: &mut [i32], p: i32, q: i32) {
    if n <= 0 {
        return;
    }
    if p < q {
        let mut i = 0;
        while i < n {
            let val = base[(q + i) as usize];
            base[(p + i) as usize] = val;
            i += 1;
        }
    } else if p > q {
        let mut i = n;
        while i > 0 {
            i -= 1;
            let val = base[(q + i) as usize];
            base[(p + i) as usize] = val;
        }
    } else {
        return;
    }
}

fn ranges_overlap(_base: &[i32], n_total: i32, p: i32, q: i32, n: i32) -> bool {
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

fn call_f_checked(base: &mut [i32], n_total: i32, n: i32, p: i32, q: i32) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, base, p, q);
    } else {
        f(n, base, p, q);
    }
}

fn init(x: &mut [i32]) {
    let mut i = 0;
    let n = x.len() as i32;
    while i < n {
        x[i as usize] = i * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    let mut i = 0;
    let n = x.len() as i32;
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

    init(&mut d);
    init(&mut expect);

    safe_move(50, &mut expect, 50, 0);
    safe_move(50, &mut expect, 1, 0);

    g_defined(&mut d);

    if !same(&d, &expect) {
        exit(1);
    }
    exit(0);
}