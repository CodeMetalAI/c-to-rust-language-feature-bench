static mut G_OFF: i32 = 9;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    let n = n.min(p.len()).min(q.len());
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    let n = n.min(a.len()).min(b.len());
    for i in 0..n {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn main() -> i32 {
    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    unsafe {
        let off = G_OFF as usize;
        let n = 8;

        let p = &mut buf[0..];
        let q = &buf[off..];

        f(n, p, q);

        if !check_range_eq(&buf[0..], &snapshot[off..], n) {
            return 1;
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
            return 2;
        }

        if buf[off - 1] != snapshot[off - 1] {
            return 3;
        }

        if buf[off + n] != snapshot[off + n] {
            return 4;
        }
    }

    unsafe {
        let off = G_OFF as usize;
        let n = 8;

        let p = &mut buf[off..];
        let q = &buf[0..];

        f(n, p, q);

        if !check_range_eq(&buf[off..], &buf[0..], n) {
            return 5;
        }
    }

    0
}