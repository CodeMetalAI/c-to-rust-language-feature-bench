fn f(n: usize, p: &mut [i32], q: &[i32]) {
    let mut p = p.iter_mut();
    let mut q = q.iter();

    for _ in 0..n {
        *p.next().unwrap() = *q.next().unwrap();
    }
}

fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
    if n == 0 {
        return;
    }

    if p.as_ptr() < q.as_ptr() {
        for i in 0..n {
            p[i] = q[i];
        }
    } else if p.as_ptr() > q.as_ptr() {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: i32) -> bool {
    let ps = (p.as_ptr() as usize) - (base.as_ptr() as usize);
    let qs = (q.as_ptr() as usize) - (base.as_ptr() as usize);

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n as usize > n_total || qs + n as usize > n_total {
        return false;
    }

    (ps < qs + n as usize) && (qs < ps + n as usize)
}

fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n as i32) {
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
    x.iter().zip(y.iter()).all(|(a, b)| a == b)
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, &mut d[50..], &d);
    call_f_checked(d, 100, 50, &mut d[1..51], &d);
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..51], &expect);

    g_defined(&mut d);

    if!same(&d, &expect, 100) {
        std::process::exit(1);
    }
}