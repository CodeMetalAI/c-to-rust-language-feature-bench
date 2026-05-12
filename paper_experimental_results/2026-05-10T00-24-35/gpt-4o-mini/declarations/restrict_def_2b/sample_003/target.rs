static mut GATE: i32 = 1;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i] = q[i];
        i += 1;
    }
}

fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
    if n == 0 {
        return;
    }

    if p.as_ptr() < q.as_ptr() {
        let mut i = 0;
        while i < n {
            p[i] = q[i];
            i += 1;
        }
    } else if p.as_ptr() > q.as_ptr() {
        let mut i = n;
        while i > 0 {
            i -= 1;
            p[i] = q[i];
        }
    }
}

fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: usize) -> bool {
    let gate = unsafe { GATE };
    let ps = p.as_ptr() as usize - base.as_ptr() as usize;
    let qs = q.as_ptr() as usize - base.as_ptr() as usize;

    if gate != 0 {
        // Simulating the "volatile" nature of gate
    }

    if n < 0 || ps < 0 || qs < 0 || ps + n > n_total as usize || qs + n > n_total as usize {
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

fn init(x: &mut [i32]) {
    for (i, value) in x.iter_mut().enumerate() {
        *value = (i * 13) + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x.iter().zip(y.iter()).all(|(a, b)| a == b)
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, &mut d[50..], &d);
    call_f_checked(d, 100, 50, &mut d[1..], &d);
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d);
    init(&mut expect);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    g_defined(&mut d);

    if !same(&d, &expect) {
        std::process::exit(1);
    }

    std::process::exit(0);
}