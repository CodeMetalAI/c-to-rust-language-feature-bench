use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

static GATE: AtomicI32 = AtomicI32::new(1);
static D: Mutex<[i32; 100]> = Mutex::new([0; 100]);

fn f(n: i32, base: &mut [i32], p: i32, q: i32) {
    let mut n = n;
    let mut pi = p as usize;
    let mut qi = q as usize;
    while n > 0 {
        base[pi] = base[qi];
        pi += 1;
        qi += 1;
        n -= 1;
    }
}

fn safe_move(n: i32, base: &mut [i32], p: i32, q: i32) {
    if n <= 0 {
        return;
    }
    let n_usize = n as usize;
    let p = p as usize;
    let q = q as usize;

    if p < q {
        let mut i = 0;
        while i < n_usize {
            base[p + i] = base[q + i];
            i += 1;
        }
    } else if p > q {
        let mut i = n_usize;
        while i > 0 {
            i -= 1;
            base[p + i] = base[q + i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(n_total: i32, p: i32, q: i32, n: i32) -> bool {
    let g = GATE.load(Ordering::SeqCst);
    let mut ps = p;
    let mut qs = q;

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

fn call_f_checked(base: &mut [i32], n_total: i32, n: i32, p: i32, q: i32) {
    if ranges_overlap(n_total, p, q, n) {
        safe_move(n, base, p, q);
    } else {
        f(n, base, p, q);
    }
}

fn init(x: &mut [i32], n: i32) {
    let mut i = 0;
    while i < n {
        x[i as usize] = i * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    let mut i = 0;
    while i < n {
        if x[i as usize] != y[i as usize] {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined() {
    let mut d = D.lock().unwrap();
    call_f_checked(&mut d[..], 100, 50, 50, 0);
    call_f_checked(&mut d[..], 100, 50, 1, 0);
}

fn main() {
    let mut expect = [0i32; 100];

    {
        let mut d = D.lock().unwrap();
        init(&mut d[..], 100);
    }
    init(&mut expect[..], 100);

    safe_move(50, &mut expect[..], 50, 0);
    safe_move(50, &mut expect[..], 1, 0);

    g_defined();

    let d = D.lock().unwrap();
    if !same(&d[..], &expect[..], 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}