// restrict_def_2b.rs

static mut GATE: bool = true;
static mut D: [i32; 100] = [0; 100];

fn f(n: usize, p: &mut [i32], q: &mut [i32]) {
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

fn ranges_overlap(base: &[i32], p: &[i32], q: &[i32], n: usize) -> bool {
    let g = unsafe { GATE };
    let ps = p.as_ptr() as isize - base.as_ptr() as isize;
    let qs = q.as_ptr() as isize - base.as_ptr() as isize;

    if g {
        ps + 0;
        qs + 0;
    }

    if n == 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if (ps + n as isize) > base.len() as isize || (qs + n as isize) > base.len() as isize {
        return false;
    }

    (ps < qs + n as isize) && (qs < ps + n as isize)
}

fn call_f_checked(base: &mut [i32], n: usize, p: &mut [i32], q: &[i32]) {
    if ranges_overlap(base, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, &mut q.to_vec());
    }
}

fn init(x: &mut [i32]) {
    for i in 0..x.len() {
        x[i] = i as i32 * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    for i in 0..x.len() {
        if x[i] != y[i] {
            return false;
        }
    }
    true
}

fn g_defined() {
    unsafe {
        call_f_checked(&mut D, 50, &mut D[50..], &D[..50]);
        call_f_checked(&mut D, 50, &mut D[1..51], &D[..50]);
    }
}

fn main() {
    let mut expect = [0; 100];

    unsafe {
        init(&mut D);
        init(&mut expect);

        safe_move(50, &mut expect[50..], &expect[..50]);
        safe_move(50, &mut expect[1..51], &expect[..50]);

        g_defined();

        if !same(&D, &expect) {
            std::process::exit(1);
        }
    }
}