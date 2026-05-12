fn f(d: &mut [i32], n: usize, p_start: usize, q_start: usize) {
    for i in 0..n {
        d[p_start + i] = d[q_start + i];
    }
}

fn safe_move(d: &mut [i32], n: usize, p_start: usize, q_start: usize) {
    if n == 0 {
        return;
    }
    if p_start < q_start {
        for i in 0..n {
            d[p_start + i] = d[q_start + i];
        }
    } else if p_start > q_start {
        for i in (0..n).rev() {
            d[p_start + i] = d[q_start + i];
        }
    }
    // else: p_start == q_start, do nothing
}

fn ranges_overlap(base: &[i32], n_total: usize, p_start: usize, q_start: usize, n: usize) -> bool {
    let ps = p_start as i32;
    let qs = q_start as i32;
    if (n as i32) < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + (n as i32) > (n_total as i32) || qs + (n as i32) > (n_total as i32) {
        return false;
    }
    (ps < qs + (n as i32)) && (qs < ps + (n as i32))
}

fn call_f_checked(d: &mut [i32], n_total: usize, n: usize, p_start: usize, q_start: usize) {
    if ranges_overlap(d, n_total, p_start, q_start, n) {
        safe_move(d, n, p_start, q_start);
    } else {
        f(d, n, p_start, q_start);
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

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() -> i32 {
    let mut d: [i32; 100] = [0; 100];
    let mut expect: [i32; 100] = [0; 100];
    init(&mut d, 100);
    init(&mut expect, 100);
    safe_move(&mut expect, 50, 50, 0);
    safe_move(&mut expect, 50, 1, 0);
    g_defined(&mut d);
    if !same(&d, &expect, 100) {
        return 1;
    }
    0
}