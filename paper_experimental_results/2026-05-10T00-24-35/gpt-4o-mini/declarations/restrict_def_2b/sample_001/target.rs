// restrict_def_2b.rs

static mut GATE: i32 = 1;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    let mut n = n;
    let p_len = p.len();
    let q_len = q.len();
    while n > 0 {
        if p.len() > p_len {
            p[p_len - n] = q[q_len - n];
        }
        n -= 1;
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
    }
}

fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: usize) -> bool {
    let g = unsafe { GATE };
    let ps = p.as_ptr() as usize - base.as_ptr() as usize;
    let qs = q.as_ptr() as usize - base.as_ptr() as usize;

    if g != 0 {
        // Do nothing
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n > n_total || qs + n > n_total {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

const D: usize = 100;
static mut D_ARRAY: [i32; D] = [0; D];

fn init(x: &mut [i32], n: usize) {
    for i in 0..n {
        x[i] = (i * 13) + 5;
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
    unsafe {
        call_f_checked(&D_ARRAY, D, 50, &mut D_ARRAY[50..], &D_ARRAY);
        call_f_checked(&D_ARRAY, D, 50, &mut D_ARRAY[1..], &D_ARRAY);
    }
}

fn main() -> i32 {
    let mut expect = [0; D];

    unsafe {
        init(&mut D_ARRAY, D);
        init(&mut expect, D);

        safe_move(50, &mut expect[50..], &expect);
        safe_move(50, &mut expect[1..], &expect);

        g_defined();

        if !same(&D_ARRAY, &expect, D) {
            return 1;
        }
    }

    0
}