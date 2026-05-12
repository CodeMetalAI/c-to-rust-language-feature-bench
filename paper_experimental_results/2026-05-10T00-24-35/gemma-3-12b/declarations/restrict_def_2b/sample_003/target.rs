fn main() {
    let mut d = [0; 100];
    let mut expect = [0; 100];

    for i in 0..100 {
        d[i] = i * 13 + 5;
        expect[i] = i * 13 + 5;
    }

    for i in 50..100 {
        expect[i] = expect[i - 50];
    }

    for i in 1..100 {
        expect[i] = expect[i - 1];
    }

    fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
        if n <= 0 {
            return;
        }

        if p.len() > q.len() {
            for i in 0..n {
                p[i] = q[i];
            }
        } else if p.len() < q.len() {
            for i in (0..n).rev() {
                p[i] = q[i];
            }
        } else {
            return;
        }
    }

    fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: usize) -> bool {
        if n < 0 {
            return false;
        }
        let ps = (p.as_ptr() as usize - base.as_ptr() as usize) as usize;
        let qs = (q.as_ptr() as usize - base.as_ptr() as usize) as usize;

        if ps < 0 || qs < 0 {
            return false;
        }
        if ps + n > n_total || qs + n > n_total {
            return false;
        }

        ps < qs + n && qs < ps + n
    }

    fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: &mut [i32], q: &[i32]) {
        if ranges_overlap(base, n_total, p, q, n) {
            safe_move(n, p, q);
        } else {
            unsafe {
                let mut p_ptr = p.as_mut_ptr();
                let mut q_ptr = q.as_ptr();
                for _ in 0..n {
                    *p_ptr = *q_ptr;
                    p_ptr = p_ptr.offset(1);
                    q_ptr = q_ptr.offset(1);
                }
            }
        }
    }

    fn g_defined() {
        call_f_checked(&d, 100, 50, &mut d[50..], &d[..]);
        call_f_checked(&d, 100, 50, &mut d[1..], &d[..]);
    }

    g_defined();

    fn same(x: &[i32], y: &[i32], n: usize) -> bool {
        for i in 0..n {
            if x[i] != y[i] {
                return false;
            }
        }
        true
    }

    if !same(&d, &expect, 100) {
        return;
    }
}