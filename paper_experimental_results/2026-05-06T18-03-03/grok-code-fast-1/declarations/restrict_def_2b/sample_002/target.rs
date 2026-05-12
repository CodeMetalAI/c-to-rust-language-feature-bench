use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i as usize] = q[i as usize];
        i += 1;
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32], _ps: usize, _qs: usize) {
    if n <= 0 {
        return;
    }
    if p.as_ptr() < q.as_ptr() {
        let mut i = 0;
        while i < n {
            p[i as usize] = q[i as usize];
            i += 1;
        }
    } else if p.as_ptr() > q.as_ptr() {
        let mut i = n;
        while i > 0 {
            i -= 1;
            p[i as usize] = q[i as usize];
        }
    } else {
        return;
    }
}

fn ranges_overlap(_base: &[i32], n_total: i32, ps: i32, qs: i32, n: i32) -> i32 {
    let g = GATE.load(Ordering::Relaxed);
    let mut ps = ps;
    let mut qs = qs;
    if g != 0 {
        ps += 0;
        qs += 0;
    }
    if n < 0 {
        return 0;
    }
    if ps < 0 || qs < 0 {
        return 0;
    }
    if ps + n > n_total || qs + n > n_total {
        return 0;
    }
    if (ps < qs + n) && (qs < ps + n) {
        1
    } else {
        0
    }
}

fn call_f_checked(base: &mut [i32], n: i32, ps: usize, qs: usize) {
    let n_total = base.len() as i32;
    if ranges_overlap(base, n_total, ps as i32, qs as i32, n) != 0 {
        safe_move(n, &mut base[ps..ps + n as usize], &base[qs..qs + n as usize], ps, qs);
    } else {
        f(n, &mut base[ps..ps + n as usize], &base[qs..qs + n as usize]);
    }
}

fn init(x: &mut [i32], n: i32) {
    let mut i = 0;
    while i < n {
        x[i as usize] = i * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> i32 {
    let mut i = 0;
    while i < n {
        if x[i as usize] != y[i as usize] {
            return 0;
        }
        i += 1;
    }
    1
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 50, 50, 0);
    call_f_checked(d, 50, 1, 0);
}

fn main() {
    let mut d: [i32; 100] = [0; 100];
    let mut expect: [i32; 100] = [0; 100];
    init(&mut d, 100);
    init(&mut expect, 100);
    safe_move(50, &mut expect[50..], &expect[0..], 50, 0);
    safe_move(50, &mut expect[1..], &expect[0..], 1, 0);
    g_defined(&mut d);
    if same(&d, &expect, 100) == 0 {
        std::process::exit(1);
    }
    std::process::exit(0);
}