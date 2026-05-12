fn f(n: i32, p: &mut [i32], q: &[i32]) {
    if p.len() < n as usize || q.len() < n as usize {
        return;
    }
    for i in 0..n as usize {
        p[i] = q[i];
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    let p_start = p.as_ptr() as usize;
    let q_start = q.as_ptr() as usize;

    if p_start < q_start {
        for i in 0..n as usize {
            p[i] = q[i];
        }
    } else if p_start > q_start {
        for i in (0..n as usize).rev() {
            p[i] = q[i];
        }
    }
}

fn ranges_overlap(base: &[i32], p: &[i32], q: &[i32], n: i32) -> bool {
    let n_total = base.len() as i32;
    let ps = p.as_ptr() as isize - base.as_ptr() as isize;
    let qs = q.as_ptr() as isize - base.as_ptr() as isize;

    if n < 0 || ps < 0 || qs < 0 || (ps + n > n_total) || (qs + n > n_total) {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(base: &mut [i32], n: i32, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

fn init(x: &mut [i32], n: usize) {
    for i in 0..n {
        x[i] = i as i32 * 13 + 5;
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

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    call_f_checked(&mut d, 50, &mut d[50..], &d);
    call_f_checked(&mut d, 50, &mut d[1..], &d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }
}