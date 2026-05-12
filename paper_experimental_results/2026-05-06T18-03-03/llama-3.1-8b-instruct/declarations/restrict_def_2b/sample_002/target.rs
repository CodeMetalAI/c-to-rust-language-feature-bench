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

fn ranges_overlap(base: &i32, n_total: i32, p: &i32, q: &i32, n: i32) -> bool {
    let mut g = unsafe { GATE };
    let ps = p as isize - base as isize;
    let qs = q as isize - base as isize;

    if g != 0 {
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

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut i32, n: i32) {
    let mut i = 0;
    while i < n {
        x.offset(i) = i * 13 + 5;
        i += 1;
    }
}

fn same(x: &i32, y: &i32, n: i32) -> bool {
    let mut i = 0;
    while i < n {
        if x.offset(i) != y.offset(i) {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined() {
    call_f_checked(&D as *const _, 100, 50, &mut D[50] as *mut _, &mut D[50] as *mut _);
    call_f_checked(&D as *const _, 100, 50, &mut D[1] as *mut _, &mut D[0] as *mut _);
}

fn main() {
    let mut expect: [i32; 100] = [0; 100];

    init(&mut D as *mut _, 100);
    init(&mut expect as *mut _, 100);

    safe_move(50, &mut expect[50] as *mut _, &mut expect[50] as *const _);
    safe_move(50, &mut expect[1] as *mut _, &mut expect[0] as *const _);

    g_defined();

    if !same(&D as *const _, &expect as *const _, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}