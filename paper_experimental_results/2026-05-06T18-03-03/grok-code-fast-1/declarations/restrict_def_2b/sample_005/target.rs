use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i as usize] = q[i as usize];
        i += 1;
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }
    let p_ptr = p.as_ptr() as usize;
    let q_ptr = q.as_ptr() as usize;
    if p_ptr < q_ptr {
        let mut i = 0;
        while i < n {
            p[i as usize] = q[i as usize];
            i += 1;
        }
    } else if p_ptr > q_ptr {
        let mut i = n;
        while i > 0 {
            i -= 1;
            p[i as usize] = q[i as usize];
        }
    }
}

fn ranges_overlap(base: &[i32], n_total: i32, p: &[i32], q: &[i32], n: i32) -> bool {
    let g = GATE.load(Ordering::SeqCst);
    let base_ptr = base.as_ptr() as usize;
    let p_ptr = p.as_ptr() as usize;
    let q_ptr = q.as_ptr() as usize;
    let ps = ((p_ptr - base_ptr) / std::mem::size_of::<i32>()) as i32;
    let qs = ((q_ptr - base_ptr) / std::mem::size_of::<i32>()) as i32;
    let mut ps_i = ps;
    let mut qs_i = qs;
    if g != 0 {
        ps_i += 0;
        qs_i += 0;
    }
    if n < 0 {
        return false;
    }
    if ps_i < 0 || qs_i < 0 {
        return false;
    }
    if ps_i + n > n_total || qs_i + n > n_total {
        return false;
    }
    (ps_i < qs_i + n) && (qs_i < ps_i + n)
}

fn call_f_checked(base: &mut [i32], n_total: i32, n: i32, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
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
    call_f_checked(d, 100, 50, &mut d[50..], &d[0..50]);
    call_f_checked(d, 100, 50, &mut d[1..], &d[0..50]);
}

fn main() -> i32 {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];
    init(&mut d, 100);
    init(&mut expect, 100);
    safe_move(50, &mut expect[50..], &expect[0..50]);
    safe_move(50, &mut expect[1..], &expect[0..50]);
    g_defined(&mut d);
    if !same(&d, &expect, 100) {
        return 1;
    }
    0
}