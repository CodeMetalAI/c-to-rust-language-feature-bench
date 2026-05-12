fn f(n: usize, p: &mut [i32], q: &mut [i32]) {
    let mut p_iter = p.iter_mut();
    let mut q_iter = q.iter();

    for _ in 0..n {
        *p_iter.next().unwrap() = *q_iter.next().unwrap();
    }
}

fn check_range_eq(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b).all(|(a, b)| a == b)
}

fn main() {
    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    let g_off = 9;

    {
        let n = 8;

        let p = &mut buf[..];
        let q = &mut buf[g_off..];

        f(n, &mut p[..n], &mut q[..n]);

        if!check_range_eq(&buf[..n], &snapshot[g_off..g_off + n]) {
            std::process::exit(1);
        }

        if!check_range_eq(&buf[g_off..g_off + n], &snapshot[g_off..g_off + n]) {
            std::process::exit(2);
        }

        if buf[g_off - 1]!= snapshot[g_off - 1] {
            std::process::exit(3);
        }

        if buf[g_off + n]!= snapshot[g_off + n] {
            std::process::exit(4);
        }
    }

    {
        let n = 8;

        let p = &mut buf[g_off..];
        let q = &mut buf[..];

        f(n, &mut p[..n], &mut q[..n]);

        if!check_range_eq(&buf[g_off..g_off + n], &buf[..n]) {
            std::process::exit(5);
        }
    }
}