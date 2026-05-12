fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect[..]);
    safe_move(50, &mut expect[1..], &expect[..]);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        return 1;
    }

    0
}

fn init(x: &mut [i32], n: usize) {
    for i in 0..n {
        x[i] = (i * 13 + 5) as i32;
    }
}

fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    if p.len() > q.len() {
        for i in 0..n {
            p[i] = q[i];
        }
    } else if p.len() < q.len() {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    }
}

fn ranges_overlap(
    base: &[i32],
    n_total: usize,
    p: &[i32],
    q: &[i32],
    n: usize,
) -> bool {
    let ps = p.as_ptr() as usize - base.as_ptr() as usize;
    let qs = q.as_ptr() as usize - base.as_ptr() as usize;

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n > n_total || qs + n > n_total {
        return false;
    }

    ps < qs + n && qs < ps + n
}

fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for _ in 0..n {
        let q_val = q[0];
        let p_val = q_val;
        p[0] = p_val;
        //Next pointer increment in C++ is implicit via post-increment operator
        //Need to manually increment in rust
        //p = &mut p[1];
        //q = &q[1];
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
    call_f_checked(d, 100, 50, &mut d[50..], &d[..]);
    call_f_checked(d, 100, 50, &mut d[1..], &d[..]);
}