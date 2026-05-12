fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    a.iter().zip(b.iter()).take(n).all(|(x, y)| x == y)
}

fn main() {
    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    let g_off = 9;
    let n = 8;

    {
        let (p, q) = (&mut buf[..], &buf[g_off..]);

        f(n, p, q);

        if !check_range_eq(&buf[..n], &snapshot[g_off..g_off + n], n) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[g_off..g_off + n], &snapshot[g_off..g_off + n], n) {
            std::process::exit(2);
        }

        if buf[g_off - 1] != snapshot[g_off - 1] {
            std::process::exit(3);
        }

        if buf[g_off + n] != snapshot[g_off + n] {
            std::process::exit(4);
        }
    }

    {
        let (p, q) = (&mut buf[g_off..], &buf[..]);

        f(n, p, q);

        if !check_range_eq(&buf[g_off..g_off + n], &buf[..n], n) {
            std::process::exit(5);
        }
    }
}