fn f(n: i32, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i as usize] = q[i as usize];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: i32) -> bool {
    a.iter()
        .zip(b.iter())
        .take(n as usize)
        .all(|(&x, &y)| x == y)
}

fn main() {
    static mut G_OFF: i32 = 9;

    let mut buf: [i32; 20] = [0; 20];
    let snapshot: [i32; 20] = [0; 20];

    for i in 0..20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = unsafe { G_OFF };
        let n = 8;

        let p = &mut buf[0..];
        let q = &buf[off as usize..];

        f(n, &mut p[..n as usize], &q[..n as usize]);

        if !check_range_eq(&buf[..n as usize], &snapshot[off as usize..][..n as usize], n) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off as usize..][..n as usize], &snapshot[off as usize..][..n as usize], n) {
            std::process::exit(2);
        }

        if buf[(off - 1) as usize] != snapshot[(off - 1) as usize] {
            std::process::exit(3);
        }

        if buf[(off + n) as usize] != snapshot[(off + n) as usize] {
            std::process::exit(4);
        }
    }

    {
        let off = unsafe { G_OFF };
        let n = 8;

        let p = &mut buf[off as usize..];
        let q = &buf[..];

        f(n, &mut p[..n as usize], &q[..n as usize]);

        if !check_range_eq(&buf[off as usize..][..n as usize], &buf[..n as usize], n) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}