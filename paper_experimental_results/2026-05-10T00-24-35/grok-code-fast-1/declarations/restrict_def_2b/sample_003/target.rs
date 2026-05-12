use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn safe_move(p_start: usize, q_start: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if n == 0 {
        return;
    }
    if p_start < q_start {
        for i in 0..n {
            p[i] = q[i];
        }
    } else if p_start > q_start {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(n_total: usize, p_start: usize, q_start: usize, n: usize) -> bool {
    let g = GATE.load(Ordering::Relaxed);
    let ps = p_start as isize;
    let qs = q_start as isize;
    if g != 0 {
        // Mimic the nop additions
    }
    if n as isize < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps as usize + n > n_total || qs as usize + n > n_total {
        return false;
    }
    (ps as usize < qs as usize + n) && (qs as usize < ps as usize + n)
}

fn call_f_checked(d: &mut [i32; 100], n_total: usize, n: usize, p_start: usize, q_start: usize) {
    if ranges_overlap(n_total, p_start, q_start, n) {
        safe_move(
            p_start,
            q_start,
            n,
            &mut d[p_start..p_start + n],
            &d[q_start..q_start + n],
        );
    } else {
        f(n, &mut d[p_start..p_start + n], &d[q_start..q_start + n]);
    }
}

fn init(x: &mut [i32], n: usize) {
    for i in 0..n {
        x[i] = (i * 13 + 5) as i32;
    }
}

fn same(x: &[i32], y: &[i32], n: usize) -> bool {
    for i in 0..n {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

fn g_defined(d: &mut [i32; 100]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() -> i32 {
    let mut d: [i32; 100] = [0; 100];
    let mut expect: [i32; 100] = [0; 100];
    init(&mut d, 100);
    init(&mut expect, 100);
    safe_move(50, 0, 50, &mut expect[50..100], &expect[0..50]);
    safe_move(1, 0, 50, &mut expect[1..51], &expect[0..50]);
    g_defined(&mut d);
    if !same(&d, &expect, 100) {
        1
    } else {
        0
    }
}