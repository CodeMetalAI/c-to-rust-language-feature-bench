fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
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

fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: usize) -> bool {
    let g = 1; // equivalent to volatile int gate = 1;
    let ps = p.as_ptr().offset_from(base.as_ptr());
    let qs = q.as_ptr().offset_from(base.as_ptr());

    if g!= 0 {
        // do nothing, equivalent to ps += 0; qs += 0;
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n as isize > n_total as isize || qs + n as isize > n_total as isize {
        return false;
    }

    return (ps < qs + n as isize) && (qs < ps + n as isize);
}

fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
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
        if x[i]!= y[i] {
            return false;
        }
    }
    true
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, &mut d[50..], &d);
    call_f_checked(d, 100, 50, &mut d[1..], &d);
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    g_defined(&mut d);

    if!same(&d, &expect, 100) {
        std::process::exit(1);
    }
}