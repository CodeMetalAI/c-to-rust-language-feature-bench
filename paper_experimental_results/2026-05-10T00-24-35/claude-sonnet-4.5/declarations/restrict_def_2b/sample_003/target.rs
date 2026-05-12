static mut GATE: i32 = 1;

fn f(mut n: i32, p: &mut [i32], q: &[i32]) {
    let mut p_idx = 0;
    let mut q_idx = 0;
    while n > 0 {
        p[p_idx] = q[q_idx];
        p_idx += 1;
        q_idx += 1;
        n -= 1;
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    let n = n as usize;
    let p_ptr = p.as_ptr() as usize;
    let q_ptr = q.as_ptr() as usize;

    if p_ptr < q_ptr {
        let mut i = 0;
        while i < n {
            p[i] = q[i];
            i += 1;
        }
    } else if p_ptr > q_ptr {
        let mut i = n;
        while i > 0 {
            i -= 1;
            p[i] = q[i];
        }
    }
}

fn ranges_overlap(base: &[i32], n_total: i32, p: &[i32], q: &[i32], n: i32) -> bool {
    let g = unsafe { GATE };
    let base_ptr = base.as_ptr() as isize;
    let p_ptr = p.as_ptr() as isize;
    let q_ptr = q.as_ptr() as isize;

    let mut ps = (p_ptr - base_ptr) / std::mem::size_of::<i32>() as isize;
    let mut qs = (q_ptr - base_ptr) / std::mem::size_of::<i32>() as isize;

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
    if ps + n as isize > n_total as isize || qs + n as isize > n_total as isize {
        return false;
    }

    (ps < qs + n as isize) && (qs < ps + n as isize)
}

fn call_f_checked(base: &[i32], n_total: i32, n: i32, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

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
    unsafe {
        let d_ptr = D.as_mut_ptr();
        call_f_checked(&D, 100, 50, std::slice::from_raw_parts_mut(d_ptr.add(50), 50), std::slice::from_raw_parts(d_ptr, 50));
        call_f_checked(&D, 100, 50, std::slice::from_raw_parts_mut(d_ptr.add(1), 50), std::slice::from_raw_parts(d_ptr, 50));
    }
}

fn main() {
    let mut expect = [0i32; 100];

    unsafe {
        init(&mut D, 100);
    }
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect[..]);
    safe_move(50, &mut expect[1..], &expect[..]);

    g_defined();

    unsafe {
        if !same(&D, &expect, 100) {
            std::process::exit(1);
        }
    }
}