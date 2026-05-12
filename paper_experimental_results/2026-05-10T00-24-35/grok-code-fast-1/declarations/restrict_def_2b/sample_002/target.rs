const GATE: i32 = 1;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
    if n == 0 {
        return;
    }
    if (p.as_ptr() as usize) < (q.as_ptr() as usize) {
        for i in 0..n {
            p[i] = q[i];
        }
    } else if (p.as_ptr() as usize) > (q.as_ptr() as usize) {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    }
    // if equal, do nothing
}

fn ranges_overlap(_base: &[i32], n_total: usize, p: usize, q: usize, n: usize) -> bool {
    let g = GATE;
    let mut ps = p as i32;
    let mut qs = q as i32;
    if g != 0 {
        ps += 0;
        qs += 0;
    }
    if n as i32 < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n as i32 > n_total as i32 || qs + n as i32 > n_total as i32 {
        return false;
    }
    (ps < qs + n as i32) && (qs < ps + n as i32)
}

fn call_f_checked(base: &mut [i32], n_total: usize, n: usize, p_start: usize, q_start: usize) {
    if ranges_overlap(base, n_total, p_start, q_start, n) {
        safe_move(n, &mut base[p_start..p_start + n], &base[q_start..q_start + n]);
    } else {
        f(n, &mut base[p_start..p_start + n], &base[q_start..q_start + n]);
    }
}

fn init(x: &mut [i32], n: usize) {
    for i in 0..n {
        x[i] = (i as i32) * 13 + 5;
    }
}

fn same_arry(x: &[i32], y: &[i32], n: usize) -> bool {
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
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..100], &expect[0..50]);
    safe_move(50, &mut expect[1..51], &expect[0..50]);

    g_defined(&mut d);

    if !same_arry(&d, &expect, 100) {
        return 1;
    }

    0
}