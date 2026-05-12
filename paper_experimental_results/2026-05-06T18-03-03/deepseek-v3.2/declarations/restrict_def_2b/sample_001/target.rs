fn main() {
    static mut GATE: i32 = 1;

    fn f(n: i32, p: &mut [i32], q: &[i32]) {
        let mut n = n;
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

        let p_ptr = p.as_ptr() as usize;
        let q_ptr = q.as_ptr() as usize;
        if p_ptr < q_ptr {
            for i in 0..n as usize {
                p[i] = q[i];
            }
        } else if p_ptr > q_ptr {
            let mut i = n as usize;
            while i > 0 {
                i -= 1;
                p[i] = q[i];
            }
        } else {
            return;
        }
    }

    fn ranges_overlap(base: &[i32], p: &[i32], q: &[i32], n: i32) -> bool {
        let g = GATE;
        let base_ptr = base.as_ptr() as usize;
        let p_ptr = p.as_ptr() as usize;
        let q_ptr = q.as_ptr() as usize;

        let ps = (p_ptr - base_ptr) / std::mem::size_of::<i32>();
        let qs = (q_ptr - base_ptr) / std::mem::size_of::<i32>();

        if g != 0 {
            // ps += 0; qs += 0; // no effect
        }

        if n < 0 {
            return false;
        }
        if ps < 0 || qs < 0 {
            return false;
        }
        let n_total = base.len();
        if ps + n as usize > n_total || qs + n as usize > n_total {
            return false;
        }

        return (ps < qs + n as usize) && (qs < ps + n as usize);
    }

    fn call_f_checked(base: &[i32], n: i32, p: &mut [i32], q: &[i32]) {
        if ranges_overlap(base, p, q, n) {
            safe_move(n, p, q);
        } else {
            f(n, p, q);
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
        return true;
    }

    fn g_defined(d: &mut [i32]) {
        let n_total = d.len();
        call_f_checked(d, 50, &mut d[50..], &d[..]);
        call_f_checked(d, 50, &mut d[1..], &d[..]);
    }

    let mut d = [0; 100];
    let mut expect = [0; 100];

    init(&mut d);
    init(&mut expect);

    safe_move(50, &mut expect[50..], &expect[..]);
    safe_move(50, &mut expect[1..], &expect[..]);

    g_defined(&mut d);

    if !same(&d, &expect) {
        std::process::exit(1);
    }

    std::process::exit(0);
}