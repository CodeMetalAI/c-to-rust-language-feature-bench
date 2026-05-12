fn main() {
    let mut d: [i32; 100] = [0; 100];

    fn init(x: &mut [i32], n: usize) {
        let mut i = 0;
        while i < n {
            x[i] = i as i32 * 13 + 5;
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

    init(&mut d, 100);
    let mut expect: [i32; 100] = [0; 100];
    init(&mut expect, 100);

    let mut expect_slice_50 = &mut expect[50..];
    let mut expect_slice_1 = &mut expect[1..];

    safe_move(50, expect_slice_50, &expect);
    safe_move(50, expect_slice_1, &expect);

    fn safe_move(n: usize, p: &mut [i32], q: &[i32]) {
        if n <= 0 {
            return;
        }

        if p.len() > q.len() {
            let mut i = 0;
            while i < n {
                p[i] = q[i];
                i += 1;
            }
        } else if p.len() < q.len() {
            let mut i = n;
            while i > 0 {
                i -= 1;
                p[i] = q[i];
            }
        } else {
            return;
        }
    }

    fn ranges_overlap(
        base: &[i32],
        n_total: usize,
        p: &[i32],
        q: &[i32],
        n: usize,
    ) -> bool {
        let ps = p as *const [i32] as usize - base as *const [i32] as usize;
        let qs = q as *const [i32] as usize - base as *const [i32] as usize;

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

    fn call_f_checked(
        base: &[i32],
        n_total: usize,
        n: usize,
        p: &mut [i32],
        q: &[i32],
    ) {
        if ranges_overlap(base, n_total, p, q, n) {
            safe_move(n, p, q);
        } else {
            f(n, p, q);
        }
    }

    fn f(n: usize, p: &mut [i32], q: &[i32]) {
        let mut i = 0;
        while i < n {
            p[i] = q[i];
            i += 1;
        }
    }

    fn g_defined() {
        call_f_checked(&d, 100, 50, &mut d[50..], &d);
        call_f_checked(&d, 100, 50, &mut d[1..], &d);
    }

    g_defined();

    if !same(&d, &expect, 100) {
        return;
    }
}