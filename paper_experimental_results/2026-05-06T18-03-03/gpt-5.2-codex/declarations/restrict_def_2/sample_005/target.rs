use std::process::exit;

static G_OFF: i32 = 9;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while i < n {
        p[i] = q[i];
        i += 1;
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let mut buf = [0i32; 20];
    let mut snapshot = [0i32; 20];

    let mut i = 0;
    while i < 20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = G_OFF as usize;
        let n = 8usize;

        let (left, right) = buf.split_at_mut(off);
        f(n, &mut left[0..n], &right[0..n]);

        if !check_range_eq(&buf[0..n], &snapshot[off..off + n], n) {
            exit(1);
        }

        if !check_range_eq(&buf[off..off + n], &snapshot[off..off + n], n) {
            exit(2);
        }

        if buf[off - 1] != snapshot[off - 1] {
            exit(3);
        }

        if buf[off + n] != snapshot[off + n] {
            exit(4);
        }
    }

    {
        let off = G_OFF as usize;
        let n = 8usize;

        let (left, right) = buf.split_at_mut(off);
        f(n, &mut right[0..n], &left[0..n]);

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n) {
            exit(5);
        }
    }

    exit(0);
}