use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(base: &mut [i32], n: i32, p_idx: usize, q_idx: usize) {
    if n <= 0 {
        return;
    }
    let n_usize = n as usize;
    let temp: Vec<i32> = base[q_idx..q_idx + n_usize].to_vec();
    for i in 0..n_usize {
        base[p_idx + i] = temp[i];
    }
}

fn safe_move(base: &mut [i32], n: i32, p_idx: usize, q_idx: usize) {
    if n <= 0 {
        return;
    }
    let n_usize = n as usize;
    if p_idx < q_idx {
        let mut i = 0usize;
        while i < n_usize {
            let val = base[q_idx + i];
            base[p_idx + i] = val;
            i += 1;
        }
    } else if p_idx > q_idx {
        let mut i = n_usize;
        while i > 0 {
            i -= 1;
            let val = base[q_idx + i];
            base[p_idx + i] = val;
        }
    } else {
        return;
    }
}

fn ranges_overlap(n_total: i32, p_idx: i32, q_idx: i32, n: i32) -> bool {
    let g = GATE.load(Ordering::SeqCst);
    let mut ps = p_idx;
    let mut qs = q_idx;

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

fn call_f_checked(base: &mut [i32], n_total: i32, n: i32, p_idx: usize, q_idx: usize) {
    if ranges_overlap(n_total, p_idx as i32, q_idx as i32, n) {
        safe_move(base, n, p_idx, q_idx);
    } else {
        f(base, n, p_idx, q_idx);
    }
}

fn init(x: &mut [i32], n: i32) {
    let mut i = 0usize;
    let n_usize = n as usize;
    while i < n_usize {
        x[i] = (i as i32) * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    let mut i = 0usize;
    let n_usize = n as usize;
    while i < n_usize {
        if x[i] != y[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined(d: &mut [i32; 100]) {
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