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
    let mut g = 1;
    let ps = (p.as_ptr() as usize - base.as_ptr() as usize) as isize;
    let qs = (q.as_ptr() as usize - base.as_ptr() as usize) as isize;

    if g != 0 {
        ps as usize + 0;
        qs as usize + 0;
    }

    if n < 0 || ps < 0 || qs < 0 || ps as usize + n > n_total || qs as usize + n > n_total {
        false
    } else {
        ps as usize < qs as usize + n && qs as usize < ps as usize + n
    }
}

fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

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

fn g_defined() {
    call_f_checked(&D, 100, 50, &mut D[50..], &D);
    call_f_checked(&D, 100, 50, &mut D[1..], &D);
}

fn main() {
    let mut expect = [0; 100];

    init(&mut D, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    g_defined();

    if !same(&D, &expect, 100) {
        std::process::exit(1);
    }
}