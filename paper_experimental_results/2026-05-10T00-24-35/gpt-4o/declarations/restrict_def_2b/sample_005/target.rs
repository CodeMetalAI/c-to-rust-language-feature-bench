// restrict_def_2b.rs

static mut GATE: i32 = 1;
static mut D: [i32; 100] = [0; 100];

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
    }
}

fn ranges_overlap(base: *const i32, n_total: isize, p: *const i32, q: *const i32, n: isize) -> bool {
    let g = unsafe { GATE };
    let ps = unsafe { p.offset_from(base) } as isize;
    let qs = unsafe { q.offset_from(base) } as isize;

    if g != 0 {
        // Dummy operations to reflect the volatile behavior
        let _ = ps + 0;
        let _ = qs + 0;
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

fn call_f_checked(base: *const i32, n_total: isize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p.as_ptr(), q.as_ptr(), n as isize) {
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
    unsafe {
        call_f_checked(D.as_ptr(), 100, 50, &mut D[50..100], &D[0..50]);
        call_f_checked(D.as_ptr(), 100, 50, &mut D[1..51], &D[0..50]);
    }
}

fn main() -> i32 {
    let mut expect = [0; 100];

    init(unsafe { &mut D }, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..100], &expect[0..50]);
    safe_move(50, &mut expect[1..51], &expect[0..50]);

    g_defined();

    if !same(unsafe { &D }, &expect, 100) {
        return 1;
    }

    0
}