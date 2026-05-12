use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, data: &mut [i32], p: i32, q: i32) {
    let mut n = n;
    let mut pi = p;
    let mut qi = q;
    while n > 0 {
        data[pi as usize] = data[qi as usize];
        pi += 1;
        qi += 1;
        n -= 1;
    }
}

fn safe_move(n: i32, data: &mut [i32], p: i32, q: i32) {
    if n <= 0 {
        return;
    }

    if p < q {
        let mut i = 0;
        while i < n {
            data[(p + i) as usize] = data[(q + i) as usize];
            i += 1;
        }
    } else if p > q {
        let mut i = n;
        while i > 0 {
            i -= 1;
            data[(p + i) as usize] = data[(q + i) as usize];
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

fn call_f_checked(n_total: i32, n: i32, data: &mut [i32], p: i32, q: i32) {
    if ranges_overlap(n_total, p, q, n) {
        safe_move(n, data, p, q);
    } else {
        f(n, data, p, q);
    }
}

fn init(data: &mut [i32], n: i32) {
    let mut i = 0;
    while i < n {
        data[i as usize] = i * 13 + 5;
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

fn g_defined(data: &mut [i32]) {
    call_f_checked(100, 50, data, 50, 0);
    call_f_checked(100, 50, data, 1, 0);
}

fn main() {
    let mut d = vec![0i32; 100];
    let mut expect = vec![0i32; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect, 50, 0);
    safe_move(50, &mut expect, 1, 0);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }
}