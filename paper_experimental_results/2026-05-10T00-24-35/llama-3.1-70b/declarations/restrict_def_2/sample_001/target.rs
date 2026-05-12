fn f(n: usize, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32]) -> bool {
    a.iter().zip(b).all(|(x, y)| x == y)
}

fn main() {
    let mut buf = [0; 20];
    let snapshot = {
        let mut snapshot = [0; 20];
        for i in 0..20 {
            snapshot[i] = i as i32 * 11 + 3;
            buf[i] = snapshot[i];
        }
        snapshot
    };

    let g_off = 9;

    let n = 8;

    let p = &mut buf[0..];
    let q = &mut buf[g_off..];

    f(n, &mut p[..n], &mut q[..n]);

    if!check_range_eq(&buf[0..n], &snapshot[g_off..(g_off + n)]) {
        std::process::exit(1);
    }

    if!check_range_eq(&buf[g_off..(g_off + n)], &snapshot[g_off..(g_off + n)]) {
        std::process::exit(2);
    }

    if buf[g_off - 1]!= snapshot[g_off - 1] {
        std::process::exit(3);
    }

    if buf[g_off + n]!= snapshot[g_off + n] {
        std::process::exit(4);
    }

    let p = &mut buf[g_off..];
    let q = &mut buf[0..];

    f(n, &mut p[..n], &mut q[..n]);

    if!check_range_eq(&buf[g_off..(g_off + n)], &buf[0..n]) {
        std::process::exit(5);
    }

    std::process::exit(0);
}