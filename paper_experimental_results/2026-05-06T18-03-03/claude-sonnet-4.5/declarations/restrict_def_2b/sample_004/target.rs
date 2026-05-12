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

fn init(x: &mut [i32], n: usize) {
    let mut i = 0;
    while i < n {
        x[i] = (i as i32) * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if x[i] != y[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined() {
    unsafe {
        let (left, right) = D.split_at_mut(50);
        call_f_checked(&D, 100, 50, right, left);
        
        let (first, rest) = D.split_at_mut(1);
        call_f_checked(&D, 100, 50, rest, first);
    }
}

fn main() {
    let mut expect = [0i32; 100];

    unsafe {
        init(&mut D, 100);
    }
    init(&mut expect, 100);

    let (left, right) = expect.split_at_mut(50);
    safe_move(50, right, left);
    
    let (first, rest) = expect.split_at_mut(1);
    safe_move(50, rest, first);

    g_defined();

    unsafe {
        if !same(&D, &expect, 100) {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}