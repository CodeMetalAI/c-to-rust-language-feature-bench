use std::process::exit;

const G_OFF: i32 = 9;

fn f(n: usize, buf: &mut [i32], p: usize, q: usize) {
    let mut i = 0usize;
    while i < n {
        let val = buf[q + i];
        buf[p + i] = val;
        i += 1;
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    let mut i = 0usize;
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

    let mut i = 0usize;
    while i < 20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = G_OFF as usize;
        let n = 8usize;

        f(n, &mut buf, 0, off);

        if !check_range_eq(&buf[0..], &snapshot[off..], n) {
            exit(1);
        }
        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
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

        f(n, &mut buf, off, 0);

        if !check_range_eq(&buf[off..], &buf[0..], n) {
            exit(5);
        }
    }

    exit(0);
}