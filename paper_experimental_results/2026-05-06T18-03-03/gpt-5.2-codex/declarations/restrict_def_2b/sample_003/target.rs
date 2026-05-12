use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(base: &mut [i32], mut n: i32, mut p: i32, mut q: i32) {
    while n > 0 {
        let val = base[q as usize];
        base[p as usize] = val;
        p += 1;
        q += 1;
        n -= 1;
    }
}

fn safe_move(base: &mut [i32], n: i32, p: i32, q: i32) {
    if n <= 0 {
        return;
    }
    let n_usize = n as usize;
    if p < q {
        let p_usize = p as usize;
        let q_usize = q as usize;
        let mut i = 0usize;
        while i < n_usize {
            let val = base[q_usize + i];
            base[p_usize + i] = val;
            i += 1;
        }
    } else if p > q {
        let p_usize = p as usize;
        let q_usize = q as usize;
        let mut i = n_usize;
        while i > 0 {
            i -= 1;
            let val = base[q_usize + i];
            base[p_usize + i] = val;
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

fn call_f_checked(base: &mut [i32], n_total: i32, n: i32, p: i32, q: i32) {
    if ranges_overlap(n_total, p, q, n) {
        safe_move(base, n, p, q);
    } else {
        f(base, n, p, q);
    }
}

fn init(x: &mut [i32], n: i32) {
    let mut i = 0usize;
    while i < n as usize {
        x[i] = i as i32 * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    let mut i = 0usize;
    while i < n as usize {
        if x[i] != y[i] {
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

    safe_move(&mut expect, 50, 50, 0);
    safe_move(&mut expect, 50, 1, 0);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }
}