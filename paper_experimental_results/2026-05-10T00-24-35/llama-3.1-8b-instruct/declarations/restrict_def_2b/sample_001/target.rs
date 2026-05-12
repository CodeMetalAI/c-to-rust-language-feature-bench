use std::mem;

const GATE: i32 = 1;

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

    let base = unsafe { p.as_ptr() };
    let q_base = q as *const i32;

    if q_base < base {
        let i = 0;
        while i < n {
            unsafe { *p.offset(i) = *q.offset(i); }
            i += 1;
        }
    } else if q_base > base {
        let i = n;
        while i > 0 {
            unsafe { *p.offset(-(i as isize)) = *q.offset(-(i as isize)); }
            i -= 1;
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: *const i32, n_total: i32, p: *const i32, q: *const i32, n: i32) -> bool {
    let g = GATE;
    let ps = (p as isize - base as isize) as i32;
    let qs = (q as isize - base as isize) as i32;

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
    if ps + n > n_total || qs + n > n_total {
        return false;
    }

    return ps < qs + n && qs < ps + n;
}

fn call_f_checked(base: *const i32, n_total: i32, n: i32, p: &mut i32, q: &mut i32) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut i32, n: i32) {
    let mut i = 0;
    while i < n {
        *x = i * 13 + 5;
        x = x.offset(1);
        i += 1;
    }
}

fn same(x: &const i32, y: &const i32, n: i32) -> bool {
    let mut i = 0;
    while i < n {
        if unsafe { *x.offset(i)!= *y.offset(i) } {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined() {
    call_f_checked(&D as *const _, 100, 50, &mut D[50], &mut D);
    call_f_checked(&D as *const _, 100, 50, &mut D[1], &mut D);
}

fn main() {
    let mut expect: [i32; 100] = [0; 100];

    init(&mut D as *mut _, 100);
    init(&mut expect as *mut _, 100);

    safe_move(50, &mut expect[50], &mut expect);
    safe_move(50, &mut expect[1], &mut expect);

    g_defined();

    if!same(&D as *const _, &expect as *const _, 100) {
        panic!();
    }
}