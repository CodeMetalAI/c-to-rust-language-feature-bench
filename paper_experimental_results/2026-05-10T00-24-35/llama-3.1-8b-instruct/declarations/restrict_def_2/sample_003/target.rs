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

        let mut p = &mut buf[..];
        let mut q = &mut buf[off..];

        mem::swap(&mut p, &mut q);

        if!check_range_eq(&buf[..n], &snapshot[off..], n) {
            panic!("failed check_range_eq 1");
        }

        if!check_range_eq(&buf[off..], &snapshot[off..], n) {
            panic!("failed check_range_eq 2");
        }

        if buf[off - 1]!= snapshot[off - 1] {
            panic!("failed check off - 1");
        }

        if buf[off + n]!= snapshot[off + n] {
            panic!("failed check off + n");
        }
    }

    {
        let off = 9;
        let n = 8;

        let mut p = &mut buf[off..];
        let mut q = &mut buf[..];

        mem::swap(&mut p, &mut q);

        if!check_range_eq(&buf[off..n], &buf[..], n) {
            panic!("failed check_range_eq 3");
        }
    }

    assert!(true);
}