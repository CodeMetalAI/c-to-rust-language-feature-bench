fn main() {
    static mut GATE: i32 = 1;

    fn f(n: i32, p: &mut [i32], q: &[i32]) {
        let mut n = n;
        let mut i = 0;
        while n > 0 {
            p[i] = q[i];
            i += 1;
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
            for i in 0..n {
                p[i as usize] = q[i as usize];
            }
        } else if p_ptr > q_ptr {
            for i in (0..n).rev() {
                p[i as usize] = q[i as usize];
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
            // ps += 0; qs += 0; // No effect, so omitted
        }

        if n < 0 {
            return false;
        }
        if ps < 0 || qs < 0 {
            return false;
        }
        let n_total = base.len() as i32;
        if ps + n > n_total || qs + n > n_total {
            return false;
        }

        return (ps < qs + n) && (qs < ps + n);
    }

    fn call_f_checked(base: &[i32], n: i32, p: &mut [i32], q: &[i32]) {
        if ranges_overlap(base, p, q, n) {
            safe_move(n, p, q);
        } else {
            f(n, p, q);
        }
    }

    static mut D: [i32; 100] = [0; 100];

    fn init(x: &mut [i32], n: i32) {
        for i in 0..n {
            x[i as usize] = i * 13 + 5;
        }
    }

    fn same(x: &[i32], y: &[i32], n: i32) -> bool {
        for i in 0..n {
            if x[i as usize] != y[i as usize] {
                return false;
            }
        }
        return true;
    }

    fn g_defined() {
        let base = &mut D[..];
        call_f_checked(base, 50, &mut D[50..], &D[..]);
        call_f_checked(base, 50, &mut D[1..], &D[..]);
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

    std::process::exit(0);
}