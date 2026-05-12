const GATE: i32 = 1;

fn f(d: &mut [i32], p_idx: usize, q_idx: usize, n: usize) {
    d[p_idx..p_idx + n].copy_from_slice(&d[q_idx..q_idx + n]);
}

fn safe_move(d: &mut [i32], p_idx: usize, q_idx: usize, n: usize) {
    if n == 0 {
        return;
    }
    if p_idx < q_idx {
        for i in 0..n {
            d[p_idx + i] = d[q_idx + i];
        }
    } else if p_idx > q_idx {
        for i in (0..n).rev() {
            d[p_idx + i] = d[q_idx + i];
        }
    } else {
        // same, do nothing
    }
}

fn ranges_overlap(n_total: usize, p_idx: usize, q_idx: usize, n: usize) -> bool {
    let g = GATE;
    let mut ps = p_idx;
    let mut qs = q_idx;
    if g != 0 {
        ps += 0;
        qs += 0;
    }
    if n == 0 {
        return false;
    }
    if ps >= n_total || qs >= n_total || ps + n > n_total || qs + n > n_total {
        return false;
    }
    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(d: &mut [i32], p_idx: usize, q_idx: usize, n: usize) {
    if ranges_overlap(d.len(), p_idx, q_idx, n) {
        safe_move(d, p_idx, q_idx, n);
    } else {
        f(d, p_idx, q_idx, n);
    }
}

fn init(x: &mut [i32]) {
    for i in 0..x.len() {
        x[i] = (i as i32) * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x == y
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 50, 0, 50);
    call_f_checked(d, 1, 0, 50);
}

fn main() {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];
    init(&mut d);
    init(&mut expect);
    safe_move(&mut expect, 50, 0, 50);
    safe_move(&mut expect, 1, 0, 50);
    g_defined(&mut d);
    if !same(&d, &expect) {
        std::process::exit(1);
    }
    std::process::exit(0);
}