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
        p[..n].copy_from_slice(&q[..n]);
    } else if p.as_ptr() > q.as_ptr() {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    }
}

fn ranges_overlap(base: &[i32], p: &[i32], q: &[i32], n: usize) -> bool {
    let ps = p.as_ptr() as isize - base.as_ptr() as isize;
    let qs = q.as_ptr() as isize - base.as_ptr() as isize;
    let n_total = base.len();

    if n == 0 || ps < 0 || qs < 0 || (ps as usize) + n > n_total || (qs as usize) + n > n_total {
        false
    } else {
        (ps as usize) < (qs as usize) + n && (qs as usize) < (ps as usize) + n
    }
}

fn call_f_checked(base: &mut [i32], n: usize, p: &mut [i32], q: &[i32]) {
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

fn same(x: &[i32], y: &[i32]) -> bool {
    x == y
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 50, &mut d[50..], &d[..50]);
    call_f_checked(d, 50, &mut d[1..51], &d[..50]);
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect[..50]);
    safe_move(50, &mut expect[1..51], &expect[..50]);

    g_defined(&mut d);

    if !same(&d, &expect) {
        std::process::exit(1);
    }
}