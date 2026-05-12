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
    } else {
        return;
    }
}

fn ranges_overlap(base: &[i32], p: &[i32], q: &[i32], n: usize) -> bool {
    let ps = p.as_ptr() as usize - base.as_ptr() as usize;
    let qs = q.as_ptr() as usize - base.as_ptr() as usize;

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n > base.len() || qs + n > base.len() {
        return false;
    }

    ps < qs + n && qs < ps + n
}

fn call_f_checked(base: &[i32], n: usize, p: &mut [i32], q: &mut [i32]) {
    if ranges_overlap(base, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut [i32]) {
    for (i, v) in x.iter_mut().enumerate() {
        *v = i as i32 * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x.iter().zip(y).all(|(a, b)| a == b)
}

fn g_defined() {
    let d = unsafe { &mut D };
    call_f_checked(d, 50, &mut d[50..], &mut d[..]);
    call_f_checked(d, 50, &mut d[1..], &mut d[..]);
}

fn main() {
    let mut expect = [0; 100];

    init(&mut expect);
    init(unsafe { &mut D });

    safe_move(50, &mut expect[50..], &expect[..]);
    safe_move(50, &mut expect[1..], &expect[..]);

    g_defined();

    if!same(&expect, unsafe { &D }, 100) {
        std::process::exit(1);
    }
}