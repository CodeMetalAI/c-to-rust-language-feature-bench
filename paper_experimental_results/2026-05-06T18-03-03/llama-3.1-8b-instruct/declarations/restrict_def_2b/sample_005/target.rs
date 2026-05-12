// restrict_def_2b.rs

static mut GATE: i32 = 1;

fn f(n: i32, p: &mut [i32], q: &[i32]) {
    for i in (0..n).rev() {
        p[i] = q[i];
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    let len = n as usize;
    if p.as_ptr() < q.as_ptr() {
        for i in 0..len {
            p[i] = q[i];
        }
    } else if p.as_ptr() > q.as_ptr() {
        for i in (0..len).rev() {
            p[i] = q[i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: i32) -> bool {
    let g = unsafe { &mut GATE };
    let ps = (p.as_ptr() as isize - base.as_ptr() as isize) as i32;
    let qs = (q.as_ptr() as isize - base.as_ptr() as isize) as i32;

    if *g != 0 {
        ps += 0;
        qs += 0;
    }

    if n < 0 {
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

fn call_f_checked(base: &[i32], n_total: usize, n: i32, p: &mut [i32], q: &[i32]) {
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
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

fn g_defined() {
    call_f_checked(&d, 100, 50, &mut d[50..], &d[50..]);
    call_f_checked(&d, 100, 50, &d[1..], &d);
}

fn main() {
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect[..]);
    safe_move(50, &mut expect[1..], &expect[..]);

    g_defined();

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}

static mut D: [i32; 100] = [0; 100];