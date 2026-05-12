fn main() {
    static mut G_OFF: i32 = 9;

    fn f(n: i32, p: &mut [i32], q: &[i32]) {
        let mut n = n as usize;
        let mut p_idx = 0;
        let mut q_idx = 0;
        while n > 0 {
            p[p_idx] = q[q_idx];
            p_idx += 1;
            q_idx += 1;
            n -= 1;
        }
    }

    fn check_range_eq(a: &[i32], b: &[i32], n: i32) -> bool {
        let n = n as usize;
        for i in 0..n {
            if a[i] != b[i] {
                return false;
            }
        }
        true
    }

    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    // SAFETY: G_OFF is only accessed in this single-threaded context
    let g_off = unsafe { G_OFF };
    {
        let off = g_off as usize;
        let n = 8;

        let p_slice = &mut buf[0..];
        let q_slice = &buf[off..];

        f(n as i32, &mut p_slice[0..n], &q_slice[0..n]);

        if !check_range_eq(&buf[0..n], &snapshot[off..off + n], n as i32) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..off + n], &snapshot[off..off + n], n as i32) {
            std::process::exit(2);
        }

        if buf[off - 1] != snapshot[off - 1] {
            std::process::exit(3);
        }

        if buf[off + n] != snapshot[off + n] {
            std::process::exit(4);
        }
    }

    {
        let off = g_off as usize;
        let n = 8;

        let p_slice = &mut buf[off..];
        let q_slice = &buf[0..];

        f(n as i32, &mut p_slice[0..n], &q_slice[0..n]);

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n as i32) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}