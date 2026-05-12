use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(arr: &mut [i32; 100], p_start: usize, q_start: usize, n: i32) {
    for i in 0..n as usize {
        arr[p_start + i] = arr[q_start + i];
    }
}

fn safe_move(arr: &mut [i32; 100], p_start: usize, q_start: usize, n: i32) {
    if n <= 0 {
        return;
    }
    if p_start < q_start {
        for i in 0..n as usize {
            arr[p_start + i] = arr[q_start + i];
        }
    } else if p_start > q_start {
        for i in (0..n as usize).rev() {
            arr[p_start + i] = arr[q_start + i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(n_total: usize, p_start: usize, q_start: usize, n: i32) -> bool {
    let g = GATE.load(Ordering::Relaxed);
    let mut ps = p_start as i32;
    let mut qs = q_start as i32;
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
    if ps + n > n_total as i32 || qs + n > n_total as i32 {
        return false;
    }
    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(arr: &mut [i32; 100], n_total: usize, n: i32, p_start: usize, q_start: usize) {
    if ranges_overlap(n_total, p_start, q_start, n) {
        safe_move(arr, p_start, q_start, n);
    } else {
        f(arr, p_start, q_start, n);
    }
}

fn init(arr: &mut [i32; 100], n: i32) {
    for i in 0..n as usize {
        arr[i] = (i * 13 + 5) as i32;
    }
}

fn same(x: &[i32; 100], y: &[i32; 100], n: i32) -> bool {
    for i in 0..n as usize {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

fn g_defined(arr: &mut [i32; 100]) {
    call_f_checked(arr, 100, 50, 50, 0);
    call_f_checked(arr, 100, 50, 1, 0);
}

fn main() {
    let mut d: [i32; 100] = [0; 100];
    let mut expect: [i32; 100] = [0; 100];
    init(&mut d, 100);
    init(&mut expect, 100);
    safe_move(&mut expect, 50, 0, 50);
    safe_move(&mut expect, 1, 0, 50);
    g_defined(&mut d);
    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }
    std::process::exit(0);
}