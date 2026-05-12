fn main() {
    let mut d: Vec<i32> = vec![0; 100];
    let mut expect: Vec<i32> = vec![0; 100];

    for i in 0..100 {
        d[i] = (i * 13 + 5) as i32;
        expect[i] = (i * 13 + 5) as i32;
    }

    let mut expect_50 = expect[50..].to_vec();
    let mut expect_1 = expect[1..].to_vec();

    safe_move(50, &mut expect_50, &expect[..50]);
    safe_move(50, &mut expect_1, &expect[..]);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        return 1;
    }

    0
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
    } else {
        return;
    }
}

fn ranges_overlap(
    base: &[i32],
    n_total: usize,
    p: &[i32],
    q: &[i32],
    n: usize,
) -> bool {
    let ps = p as *const [i32] as usize - base as *const [i32] as usize;
    let qs = q as *const [i32] as usize - base as *const [i32] as usize;

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
    let mut p_ptr = p.as_mut_ptr();
    let mut q_ptr = q.as_ptr();

    for _ in 0..n {
        unsafe {
            *p_ptr = *q_ptr;
            p_ptr = p_ptr.add(1);
            q_ptr = q_ptr.add(1);
        }
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

fn g_defined(d: &mut Vec<i32>) {
    call_f_checked(d, 100, 50, &mut d[50..], &d[..]);
    call_f_checked(d, 100, 50, &mut d[1..], &d[..]);
}