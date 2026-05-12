const GATE: VolatileAtomicUsize = VolatileAtomicUsize::new(1);

fn f(n: usize, p: *mut i32, q: *mut i32) {
    let mut p = p;
    let mut q = q;
    for _ in 0..n {
        unsafe {
            *p = *q;
            p = p.offset(1);
            q = q.offset(1);
        }
    }
}

fn safe_move(n: usize, p: *mut i32, q: *const i32) {
    if n == 0 {
        return;
    }
    let (p, q) = match (p, q) {
        (p, q) if p < q => (p as *mut i32, q as *const i32),
        (p, q) if p > q => (q as *mut i32, p as *const i32),
        _ => return,
    };
    let mut i = 0;
    while i < n {
        unsafe {
            *p = *q;
        }
        p = p.offset(1);
        q = q.offset(1);
        i += 1;
    }
}

fn ranges_overlap(base: *const i32, n_total: usize, p: *const i32, q: *const i32, n: usize) -> bool {
    let g = GATE.load(AtomicOrdering::Relaxed);
    let ps = (p as *const i32).offset_from(base) as i32;
    let qs = (q as *const i32).offset_from(base) as i32;

    if g == 1 {
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

fn call_f_checked(base: *const i32, n_total: usize, n: usize, p: *mut i32, q: *mut i32) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

fn init(x: *mut i32, n: usize) {
    let mut i = 0;
    while i < n {
        unsafe {
            *x = i * 13 + 5;
        }
        x = x.offset(1);
        i += 1;
    }
}

fn same(x: *const i32, y: *const i32, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if unsafe { *x }!= unsafe { *y } {
            return false;
        }
        x = x.offset(1);
        y = y.offset(1);
        i += 1;
    }
    true
}

fn g_defined() {
    call_f_checked(&d[0], 100, 50, &d[50], &d[50]);
    call_f_checked(&d[0], 100, 50, &d[1], &d[0]);
}

fn main() {
    let mut expect: [i32; 100] = [0; 100];

    init(&mut d[0], 100);
    init(&mut expect[0], 100);

    safe_move(50, &mut expect[50], &expect[0]);
    safe_move(50, &mut expect[1], &expect[0]);

    g_defined();

    if!same(&d[0], &expect[0], 100) {
        panic!();
    }
}

const d: [i32; 100] = [0; 100];