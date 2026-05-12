fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32]) -> bool {
    a == b
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

        let p = &mut buf[0..n];
        let q = &buf[g_off..g_off + n];

        f(n, p, q);

        if !check_range_eq(&buf[0..n], &snapshot[g_off..g_off + n]) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[g_off..g_off + n], &snapshot[g_off..g_off + n]) {
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
        let n = 8;

        let p = &mut buf[g_off..g_off + n];
        let q = &buf[0..n];

        f(n, p, q);

        if !check_range_eq(&buf[g_off..g_off + n], &buf[0..n]) {
            std::process::exit(5);
        }
    }
}