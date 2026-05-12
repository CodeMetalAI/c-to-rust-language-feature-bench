// restrict_def_2b.rs

static mut GATE: i32 = 1;

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

fn ranges_overlap(base: *const i32, n_total: usize, p: *const i32, q: *const i32, n: usize) -> bool {
    let g = unsafe { GATE };
    let ps = unsafe { p.offset_from(base) as usize };
    let qs = unsafe { q.offset_from(base) as usize };

    if g != 0 {
        // Volatile read to mimic the C code behavior
        let _ = ps + 0;
        let _ = qs + 0;
    }

    if n == 0 || ps >= n_total || qs >= n_total {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(base: *const i32, n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p.as_ptr(), q.as_ptr(), n) {
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

fn g_defined(d: &mut [i32]) {
    call_f_checked(d.as_ptr(), 100, 50, &mut d[50..100], &d[0..50]);
    call_f_checked(d.as_ptr(), 100, 50, &mut d[1..51], &d[0..50]);
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..100], &expect[0..50]);
    safe_move(50, &mut expect[1..51], &expect[0..50]);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }
}