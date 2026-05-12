// restrict_def_2b.rs

use std::sync::atomic::{AtomicI32, Ordering};

static GATE: AtomicI32 = AtomicI32::new(1);

fn f(n: i32, p: &mut i32, q: &mut i32) {
    while n > 0 {
        *p = *q;
        p = p.offset(1);
        q = q.offset(1);
        n -= 1;
    }
}

fn safe_move(n: i32, p: &mut i32, q: &const i32) {
    if n <= 0 {
        return;
    }

    if p < q {
        let i = 0;
        while i < n {
            *p = *q;
            p = p.offset(1);
            q = q.offset(1);
            i += 1;
        }
    } else if p > q {
        let i = n;
        while i > 0 {
            *p = *q;
            p = p.offset(-1);
            q = q.offset(-1);
            i -= 1;
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: &i32, n_total: i32, p: &i32, q: &i32, n: i32) -> bool {
    let g = GATE.load(Ordering::Relaxed);
    let ps = (p as *const i32 as isize - base as *const i32 as isize) as i32;
    let qs = (q as *const i32 as isize - base as *const i32 as isize) as i32;

    if g > 0 {
        ps += 0;
        qs += 0;
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

fn call_f_checked(base: &i32, n_total: i32, n: i32, p: &mut i32, q: &mut i32) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

fn init(x: &mut i32, n: i32) {
    let i = 0;
    while i < n {
        *x = i * 13 + 5;
        x = x.offset(1);
        i += 1;
    }
}

fn same(x: &i32, y: &i32, n: i32) -> bool {
    let i = 0;
    while i < n {
        if *x!= *y {
            return false;
        }
        x = x.offset(1);
        y = y.offset(1);
        i += 1;
    }
    true
}

fn g_defined() {
    call_f_checked(&d[0], 100, 50, &mut d[50], &mut d[50]);
    call_f_checked(&d[0], 100, 50, &mut d[1], &mut d[0]);
}

fn main() {
    let mut expect: [i32; 100] = [0; 100];

    init(&mut d[0], 100);
    init(&mut expect[0], 100);

    safe_move(50, &mut expect[50], &mut expect[0]);
    safe_move(50, &mut expect[1], &mut expect[0]);

    g_defined();

    if!same(&d[0], &expect[0], 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}

static mut D: [i32; 100] = [0; 100];