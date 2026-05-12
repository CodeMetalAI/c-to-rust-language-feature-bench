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
    let n_total = base.len();
    let ps = p.as_ptr() as usize - base.as_ptr() as usize;
    let qs = q.as_ptr() as usize - base.as_ptr() as usize;

    if n == 0 || ps >= n_total || qs >= n_total || ps + n > n_total || qs + n > n_total {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(base: &[i32], n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

fn init(x: &mut [i32]) {
    for i in 0..x.len() {
        x[i] = i as i32 * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x == y
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d);
    init(&mut expect);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    call_f_checked(&d, 50, &mut d[50..], &d);
    call_f_checked(&d, 50, &mut d[1..], &d);

    if !same(&d, &expect) {
        std::process::exit(1);
    }
}