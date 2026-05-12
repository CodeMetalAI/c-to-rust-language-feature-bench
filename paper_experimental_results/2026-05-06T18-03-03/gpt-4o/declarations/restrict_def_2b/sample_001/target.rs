// restrict_def_2b.rs

static mut GATE: i32 = 1;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
    if n == 0 {
        return;
    }

    if p.as_ptr() < q.as_ptr() {
        for i in 0..n {
            p[i] = q[i];
        }
    } else if p.as_ptr() > q.as_ptr() {
        for i in (0..n).rev() {
            p[i] = q[i];
        }
    }
}

fn ranges_overlap(base: &[i32], p: *const i32, q: *const i32, n: usize) -> bool {
    let g = unsafe { GATE };
    let ps = unsafe { p.offset_from(base.as_ptr()) } as usize;
    let qs = unsafe { q.offset_from(base.as_ptr()) } as usize;

    if g != 0 {
        // Volatile read simulation
        let _ = ps + 0;
        let _ = qs + 0;
    }

    if n == 0 || ps >= base.len() || qs >= base.len() {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(base: &mut [i32], n: usize, p: *mut i32, q: *const i32) {
    if ranges_overlap(base, p, q, n) {
        let p_slice = unsafe { std::slice::from_raw_parts_mut(p, n) };
        let q_slice = unsafe { std::slice::from_raw_parts(q, n) };
        safe_move(n, p_slice, q_slice);
    } else {
        let p_slice = unsafe { std::slice::from_raw_parts_mut(p, n) };
        let q_slice = unsafe { std::slice::from_raw_parts(q, n) };
        f(n, p_slice, q_slice);
    }
}

fn init(x: &mut [i32]) {
    for (i, elem) in x.iter_mut().enumerate() {
        *elem = i as i32 * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x.iter().zip(y.iter()).all(|(a, b)| a == b)
}

fn g_defined(d: &mut [i32]) {
    let (left, right) = d.split_at_mut(50);
    call_f_checked(d, 50, right.as_mut_ptr(), left.as_ptr());
    call_f_checked(d, 50, d[1..].as_mut_ptr(), d.as_ptr());
}

fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d);
    init(&mut expect);

    safe_move(50, &mut expect[50..], &expect[..50]);
    safe_move(50, &mut expect[1..51], &expect[..50]);

    g_defined(&mut d);

    if !same(&d, &expect) {
        std::process::exit(1);
    }
}