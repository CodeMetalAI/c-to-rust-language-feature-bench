fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn main() {
    let mut buf = [0; 20];
    let snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    let off = 9;
    let n = 8;

    {
        let (p, q) = buf.split_at_mut(off);
        f(n, &mut p[0..n], &q[0..n]);

        if!check_range_eq(&buf[0..n], &snapshot[off..off + n]) {
            std::process::exit(1);
        }

        if!check_range_eq(&buf[off..off + n], &snapshot[off..off + n]) {
            std::process::exit(2);
        }

        if buf[off - 1]!= snapshot[off - 1] {
            std::process::exit(3);
        }

        if buf[off + n]!= snapshot[off + n] {
            std::process::exit(4);
        }
    }

    {
        let (p, q) = buf.split_at_mut(off);
        f(n, &mut q[0..n], &p[0..n]);

        if!check_range_eq(&buf[off..off + n], &buf[0..n]) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}