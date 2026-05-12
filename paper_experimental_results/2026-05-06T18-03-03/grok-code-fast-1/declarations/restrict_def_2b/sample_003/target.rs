use std::sync::atomic::{AtomicI32, Ordering};
use std::cell::RefCell;

lazy_static::lazy_static! {
    static ref GATE: AtomicI32 = AtomicI32::new(1);
    static ref D: RefCell<[i32; 100]> = RefCell::new([0; 100]);
}

fn ranges_overlap(base: &[i32], n_total: usize, p_start: usize, q_start: usize, n: usize) -> bool {
    let g = GATE.load(Ordering::SeqCst);
    let mut ps = p_start as i32;
    let mut qs = q_start as i32;
    if g != 0 {
        ps += 0;
        qs += 0;
    }
    if n == 0 || ps < 0 || qs < 0 || ps as usize + n > n_total || qs as usize + n > n_total {
        return false;
    }
    (ps < qs + n as i32) && (qs < ps + n as i32)
}

fn safe_move(n: usize, p: &mut [i32], q: &[i32], forward: bool) {
    if forward {
        for i in 0..n {
            p[i] = q[i];
        }
    } else {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    }
}

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn call_f_checked(d: &mut [i32], n_total: usize, n: usize, p_start: usize, q_start: usize) {
    let forward = p_start < q_start;
    if ranges_overlap(d, n_total, p_start, q_start, n) {
        safe_move(n, &mut d[p_start..p_start + n], &d[q_start..q_start + n], forward);
    } else {
        f(n, &mut d[p_start..p_start + n], &d[q_start..q_start + n]);
    }
}

fn init(x: &mut [i32], n: usize) {
    for i in 0..n {
        x[i] = (i as i32) * 13 + 5;
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

fn g_defined() {
    let mut d = D.borrow_mut();
    call_f_checked(&mut *d, 100, 50, 50, 0);
    call_f_checked(&mut *d, 100, 50, 1, 0);
}

fn main() {
    let mut expect = [0i32; 100];
    {
        let mut d = D.borrow_mut();
        init(&mut *d, 100);
    }
    init(&mut expect, 100);
    safe_move(50, &mut expect[50..100], &expect[0..50], false);
    safe_move(50, &mut expect[1..51], &expect[0..50], false);
    g_defined();
    let d = D.borrow();
    if !same(&*d, &expect, 100) {
        std::process::exit(1);
    }
    std::process::exit(0);
}