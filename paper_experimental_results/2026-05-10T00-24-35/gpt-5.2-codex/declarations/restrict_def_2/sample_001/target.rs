use std::process;

const G_OFF: usize = 9;

fn f(n: usize, buf: &mut [i32], p_start: usize, q_start: usize) {
    let mut i = 0;
    while i < n {
        let val = buf[q_start + i];
        buf[p_start + i] = val;
        i += 1;
    }
}

fn check_range_eq(a: &[i32], a_start: usize, b: &[i32], b_start: usize, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if a[a_start + i] != b[b_start + i] {
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
        buf[i] = (i as i32) * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = G_OFF;
        let n = 8;

        let p = 0;
        let q = off;

        f(n, &mut buf, p, q);

        if !check_range_eq(&buf, 0, &snapshot, off, n) {
            process::exit(1);
        }

        if !check_range_eq(&buf, off, &snapshot, off, n) {
            process::exit(2);
        }

        if buf[off - 1] != snapshot[off - 1] {
            process::exit(3);
        }

        if buf[off + n] != snapshot[off + n] {
            process::exit(4);
        }
    }

    {
        let off = G_OFF;
        let n = 8;

        let p = off;
        let q = 0;

        f(n, &mut buf, p, q);

        if !check_range_eq(&buf, off, &buf, 0, n) {
            process::exit(5);
        }
    }

    process::exit(0);
}