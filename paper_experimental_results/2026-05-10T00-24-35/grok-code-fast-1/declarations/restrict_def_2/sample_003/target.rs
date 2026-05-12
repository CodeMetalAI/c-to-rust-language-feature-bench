static G_OFF: i32 = 9;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    for i in 0..n {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn main() -> i32 {
    let mut buf: [i32; 20] = [0; 20];
    let mut snapshot: [i32; 20] = [0; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        let p = &mut buf[0..n];
        let q = &buf[off..off + n];

        f(n, p, q);

        if !check_range_eq(&buf[0..n], &snapshot[off..off + n], n) {
            return 1;
        }

        if !check_range_eq(&buf[off..off + n], &snapshot[off..off + n], n) {
            return 2;
        }

        if buf[off - 1] != snapshot[off - 1] {
            return 3;
        }

        if buf[off + n] != snapshot[off + n] {
            return 4;
        }
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        let p = &mut buf[off..off + n];
        let q = &buf[0..n];

        f(n, p, q);

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n) {
            return 5;
        }
    }

    0
}