use std::mem;

fn f(n: usize, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    for i in 0..n {
        if a[i]!= b[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = 9;
        let n = 8;

        let mut p = buf.as_mut_slice();
        let mut q = &mut buf[off..];

        f(n, p, q);

        if!check_range_eq(&buf[..n], &snapshot[off..n], n) {
            panic!("check_range_eq failed");
        }

        if!check_range_eq(&buf[off..n], &snapshot[off..n], n) {
            panic!("check_range_eq failed");
        }

        if buf[off - 1]!= snapshot[off - 1] {
            panic!("buf[off - 1]!= snapshot[off - 1]");
        }

        if buf[off + n]!= snapshot[off + n] {
            panic!("buf[off + n]!= snapshot[off + n]");
        }
    }

    {
        let off = 9;
        let n = 8;

        let mut p = &mut buf[off..];
        let mut q = buf.as_mut_slice();

        f(n, p, q);

        if!check_range_eq(&buf[off..n], buf.as_slice(), n) {
            panic!("check_range_eq failed");
        }
    }
}