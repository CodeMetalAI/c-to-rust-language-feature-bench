fn main() {
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
        if p.len() < n || q.len() < n {
            return;
        }

        let p_ptr = p.as_ptr() as usize;
        let q_ptr = q.as_ptr() as usize;
        if p_ptr < q_ptr {
            for i in 0..n {
                p[i] = q[i];
            }
        } else if p_ptr > q_ptr {
            for i in (0..n).rev() {
                p[i] = q[i];
            }
        } else {
            return;
        }
    }

    fn ranges_overlap(
        base: &[i32],
        p: &[i32],
        q: &[i32],
        n: usize,
    ) -> bool {
        let g = GATE;
        let base_ptr = base.as_ptr() as usize;
        let p_ptr = p.as_ptr() as usize;
        let q_ptr = q.as_ptr() as usize;
        let ps = (p_ptr - base_ptr) / std::mem::size_of::<i32>();
        let qs = (q_ptr - base_ptr) / std::mem::size_of::<i32>();

        if g != 0 {
            // simulate the ps += 0; qs += 0; which does nothing
        }

        if ps < 0 || qs < 0 {
            return false;
        }
        let ps = ps as usize;
        let qs = qs as usize;
        if ps + n > base.len() || qs + n > base.len() {
            return false;
        }

        (ps < qs + n) && (qs < ps + n)
    }

    fn call_f_checked(base: &[i32], n: usize, p: &mut [i32], q: &[i32]) {
        if ranges_overlap(base, p, q, n) {
            safe_move(n, p, q);
        } else {
            f(n, p, q);
        }
    }

    static mut D: [i32; 100] = [0; 100];

    fn init(x: &mut [i32], n: usize) {
        for i in 0..n {
            x[i] = i as i32 * 13 + 5;
        }
    }

    fn same(x: &[i32], y: &[i32], n: usize) -> bool {
        for i in 0..n {
            if x[i] != y[i] {
                return false;
            }
        }
        true
    }

    fn g_defined() {
        let n_total = 100;
        let n = 50;
        call_f_checked(&mut D, n, &mut D[50..], &D[..]);
        call_f_checked(&mut D, n, &mut D[1..], &D[..]);
    }

    let mut expect = [0; 100];
    init(&mut D, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect[50..], &expect[..]);
    safe_move(50, &mut expect[1..], &expect[..]);

    g_defined();

    if !same(&D, &expect, 100) {
        std::process::exit(1);
    }
}