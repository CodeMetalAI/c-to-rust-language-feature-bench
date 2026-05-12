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

    if p.as_ptr() < q.as_ptr() {
        for i in 0..n as usize {
            p[i] = q[i];
        }
    } else if p.as_ptr() > q.as_ptr() {
        for i in (0..n as usize).rev() {
            p[i] = q[i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: &[i32], n_total: i32, p: &[i32], q: &[i32], n: i32) -> bool {
    let mut g = unsafe { GATE };
    let ps = (p.as_ptr() as i32 - base.as_ptr() as i32) as usize;
    let qs = (q.as_ptr() as i32 - base.as_ptr() as i32) as usize;

    if g!= 0 {
        ps += 0;
        qs += 0;
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n as usize > n_total as usize || qs + n as usize > n_total as usize {
        return false;
    }

    (ps < qs + n as usize) && (qs < ps + n as usize)
}

fn call_f_checked(base: &[i32], n_total: i32, n: i32, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

#[allow(unused)]
static mut D: [i32; 100] = [0; 100];

fn init(x: &mut [i32], n: i32) {
    for i in 0..n as usize {
        x[i] = i as i32 * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    for i in 0..n as usize {
        if x[i]!= y[i] {
            return false;
        }
    }
    true
}

fn g_defined() {
    call_f_checked(&D, 100, 50, &mut D[50..], &D);
    call_f_checked(&D, 100, 50, &mut D[1..], &D);
}

fn main() -> i32 {
    let mut expect: [i32; 100] = [0; 100];

    init(&mut D, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    g_defined();

    if!same(&D, &expect, 100) {
        return 1;
    }

    0
}