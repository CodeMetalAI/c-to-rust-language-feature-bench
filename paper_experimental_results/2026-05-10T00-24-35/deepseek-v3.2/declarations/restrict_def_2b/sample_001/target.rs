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
            let n_usize = n as usize;
            for i in (0..n_usize).rev() {
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
            // ps += 0; qs += 0; // These do nothing
        }

        if n < 0 {
            return false;
        }
        if ps < 0 || qs < 0 {
            return false;
        }
        let n_total = base.len() as i32;
        let n_usize = n as usize;
        if ps + n_usize > n_total as usize || qs + n_usize > n_total as usize {
            return false;
        }

        return (ps < qs + n_usize) && (qs < ps + n_usize);
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
        for i in 0..n as usize {
            x[i] = i * 13 + 5;
        }
    }

    fn same(x: &[i32], y: &[i32], n: i32) -> bool {
        for i in 0..n as usize {
            if x[i] != y[i] {
                return false;
            }
        }
        return true;
    }

    fn g_defined() {
        let n_total = D.len();
        call_f_checked(&D, 50, &mut D[50..], &D[0..50]);
        call_f_checked(&D, 50, &mut D[1..51], &D[0..50]);
    }

    let mut expect: [i32; 100] = [0; 100];

    unsafe {
        init(&mut D, 100);
        init(&mut expect, 100);

        safe_move(50, &mut expect[50..], &expect[0..50]);
        safe_move(50, &mut expect[1..51], &expect[0..50]);

        g_defined();

        if !same(&D, &expect, 100) {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}