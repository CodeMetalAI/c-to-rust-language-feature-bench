// restrict_def_2b.rs

static mut GATE: i32 = 1;

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
        let mut i = 0;
        while i < n {
            p.offset(i) = q.offset(i);
            i += 1;
        }
    } else if p > q {
        let mut i = n;
        while i > 0 {
            p.offset(i - 1) = q.offset(i - 1);
            i -= 1;
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: *const i32, n_total: i32, p: *const i32, q: *const i32, n: i32) -> bool {
    let mut g = unsafe { &mut GATE };
    let ps = (p as *const i32).wrapping_offset_from(base as *const i32);
    let qs = (q as *const i32).wrapping_offset_from(base as *const i32);

    if *g!= 0 {
        ps.wrapping_add(0);
        qs.wrapping_add(0);
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps.wrapping_add(n) > n_total || qs.wrapping_add(n) > n_total {
        return false;
    }

    (ps < qs.wrapping_add(n)) && (qs < ps.wrapping_add(n))
}

fn call_f_checked(base: *const i32, n_total: i32, n: i32, p: *mut i32, q: *mut i32) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut [i32], n: i32) {
    for i in 0..n {
        x[i as usize] = i * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    for i in 0..n {
        if x[i as usize]!= y[i as usize] {
            return false;
        }
    }
    true
}

fn g_defined() {
    call_f_checked(&D as *const _, 100, 50, &mut D[50] as *mut _, &mut D as *mut _);
    call_f_checked(&D as *const _, 100, 50, &mut D[1] as *mut _, &mut D as *mut _);
}

fn main() {
    let mut expect: [i32; 100] = [0; 100];

    init(&mut D, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50] as *mut _, &mut expect as *const _);
    safe_move(50, &mut expect[1] as *mut _, &mut expect as *const _);

    g_defined();

    if!same(&D, &expect, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}