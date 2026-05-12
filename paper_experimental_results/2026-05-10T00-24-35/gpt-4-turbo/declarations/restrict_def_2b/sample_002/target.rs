fn main() {
    static mut D: [i32; 100] = [0; 100];
    let mut expect = [0; 100];

    init(&mut D);
    init(&mut expect);

    safe_move(50, &mut expect[50..], &expect);
    safe_move(50, &mut expect[1..], &expect);

    g_defined();

    unsafe {
        if !same(&D, &expect) {
            std::process::exit(1);
        }
    }
}

unsafe fn f(n: isize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i as usize] = q[i as usize];
        i += 1;
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

fn ranges_overlap(base: &[i32], p: &[i32], q: &[i32], n: usize) -> bool {
    let n_total = base.len();
    let ps = p.as_ptr() as isize - base.as_ptr() as isize;
    let qs = q.as_ptr() as isize - base.as_ptr() as isize;

    if n == 0 || ps < 0 || qs < 0 || ps as usize + n > n_total || qs as usize + n > n_total {
        return false;
    }

    (ps < qs + n as isize) && (qs < ps + n as isize)
}

fn call_f_checked(base: &[i32], n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, p, q, n) {
        safe_move(n, p, q);
    } else {
        unsafe {
            f(n as isize, p, q);
        }
    }
}

fn init(x: &mut [i32]) {
    for i in 0..x.len() {
        x[i] = i as i32 * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x == y
}

fn g_defined() {
    unsafe {
        call_f_checked(&D, 50, &mut D[50..], &D);
        call_f_checked(&D, 50, &mut D[1..], &D);
    }
}