fn f(n: i32, p: &mut [i32], q: &[i32]) {
    let n = n as usize;
    assert_eq!(p.len(), n);
    assert_eq!(q.len(), n);
    for i in 0..n {
        p[i] = q[i];
    }
}

fn safe_move(n: i32, base: &mut [i32], ps: i32, qs: i32) {
    let n = n as usize;
    let ps = ps as usize;
    let qs = qs as usize;
    if n == 0 {
        return;
    }
    if ps < qs {
        for i in 0..n {
            base[ps + i] = base[qs + i];
        }
    } else if ps > qs {
        for i in (0..n).rev() {
            base[ps + i] = base[qs + i];
        }
    }
    // else same, do nothing
}

fn ranges_overlap(_base: &[i32], n_total: i32, ps: i32, qs: i32, n: i32) -> bool {
    let g: i32 = 1;
    let mut ps = ps;
    let mut qs = qs;
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

fn call_f_checked(base: &mut [i32], n_total: i32, n: i32, ps: i32, qs: i32) {
    if ranges_overlap(&base[..(n_total as usize)], n_total, ps, qs, n) {
        safe_move(n, base, ps, qs);
    } else {
        let n_usize = n as usize;
        let ps_usize = ps as usize;
        let qs_usize = qs as usize;
        f(n, &mut base[ps_usize..ps_usize + n_usize], &base[qs_usize..qs_usize + n_usize]);
    }
}

fn init(x: &mut [i32], n: i32) {
    for i in 0..(n as usize) {
        x[i] = (i as i32) * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    for i in 0..(n as usize) {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() {
    let mut d: [i32; 100] = [0; 100];
    let mut expect: [i32; 100] = [0; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect, 50, 0);
    safe_move(50, &mut expect, 1, 0);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }
    std::process::exit(0);
}