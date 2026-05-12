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

    {
        let off = 9;
        let n = 8;

        let p = &mut buf[..];
        let q = &buf[off..];

        f(n, &mut p[..n], &q[..n]);

        if!check_range_eq(&p[..n], &snapshot[off..off + n]) {
            std::process::exit(1);
        }

        if!check_range_eq(&q[..n], &snapshot[off..off + n]) {
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
        let off = 9;
        let n = 8;

        let p = &mut buf[off..];
        let q = &buf[..];

        f(n, &mut p[..n], &q[..n]);

        if!check_range_eq(&p[..n], &buf[..n]) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}