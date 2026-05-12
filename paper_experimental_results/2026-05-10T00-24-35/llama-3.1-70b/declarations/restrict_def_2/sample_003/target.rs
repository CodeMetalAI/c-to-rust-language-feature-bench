fn f(n: usize, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b).all(|(&x, &y)| x == y)
}

fn main() {
    let mut buf = [0; 20];
    let snapshot = {
        let mut snapshot = [0; 20];
        for i in 0..20 {
            snapshot[i] = (i * 11 + 3) as i32;
        }
        snapshot
    };

    for i in 0..20 {
        buf[i] = (i * 11 + 3) as i32;
    }

    let off = 9;
    let n = 8;

    {
        let (p, q) = buf.split_at_mut(off);
        f(n, &mut p[..n], &q);

        if !check_range_eq(&p[..n], &snapshot[off..]) {
            std::process::exit(1);
        }

        if !check_range_eq(&q, &snapshot[off..]) {
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
        let (q, p) = buf.split_at_mut(off);
        f(n, &mut q, &p[..n]);

        if !check_range_eq(&q, &p[..n]) {
            std::process::exit(5);
        }
    }
}