use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

static GATE: AtomicI32 = AtomicI32::new(1);
static D: Mutex<[i32; 100]> = Mutex::new([0; 100]);

fn f(n: i32, p: i32, q: i32, arr: &mut [i32]) {
    if n <= 0 {
        return;
    }
    let mut i = 0;
    while i < n {
        let val = arr[(q + i) as usize];
        arr[(p + i) as usize] = val;
        i += 1;
    }
}

fn safe_move(n: i32, p: i32, q: i32, arr: &mut [i32]) {
    if n <= 0 {
        return;
    }
    if p < q {
        let mut i = 0;
        while i < n {
            let val = arr[(q + i) as usize];
            arr[(p + i) as usize] = val;
            i += 1;
        }
    } else if p > q {
        let mut i = n - 1;
        loop {
            let val = arr[(q + i) as usize];
            arr[(p + i) as usize] = val;
            if i == 0 {
                break;
            }
            i -= 1;
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

fn call_f_checked(n_total: i32, n: i32, p: i32, q: i32, arr: &mut [i32]) {
    if ranges_overlap(n_total, p, q, n) {
        safe_move(n, p, q, arr);
    } else {
        f(n, p, q, arr);
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
    call_f_checked(100, 50, 50, 0, &mut d);
    call_f_checked(100, 50, 1, 0, &mut d);
}

fn main() {
    {
        let mut d = D.lock().unwrap();
        init(&mut d, 100);
    }

    let mut expect = [0i32; 100];
    init(&mut expect, 100);

    safe_move(50, 50, 0, &mut expect);
    safe_move(50, 1, 0, &mut expect);

    g_defined();

    let d = D.lock().unwrap();
    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}