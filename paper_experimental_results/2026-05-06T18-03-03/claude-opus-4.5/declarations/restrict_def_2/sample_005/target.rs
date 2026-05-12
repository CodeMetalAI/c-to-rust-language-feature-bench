use std::sync::atomic::{AtomicI32, Ordering};

static G_OFF: AtomicI32 = AtomicI32::new(9);

fn f(mut n: i32, p: &mut [i32], q: &[i32]) {
    let mut p_idx = 0;
    let mut q_idx = 0;
    while n > 0 {
        n -= 1;
        p[p_idx] = q[q_idx];
        p_idx += 1;
        q_idx += 1;
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: i32) -> bool {
    let mut i = 0;
    while i < n {
        if a[i as usize] != b[i as usize] {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let mut buf = [0i32; 20];
    let mut snapshot = [0i32; 20];

    let mut i = 0;
    while i < 20 {
        buf[i] = (i as i32) * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = G_OFF.load(Ordering::SeqCst) as usize;
        let n = 8;

        // Copy from buf[off..] to a temporary, then copy to buf[0..]
        let src: Vec<i32> = buf[off..off + n as usize].to_vec();
        f(n, &mut buf[0..], &src);

        if !check_range_eq(&buf[0..], &snapshot[off..], n) {
            return 1;
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
            return 2;
        }

        if buf[off - 1] != snapshot[off - 1] {
            return 3;
        }

        if buf[off + n as usize] != snapshot[off + n as usize] {
            return 4;
        }
    }

    {
        let off = G_OFF.load(Ordering::SeqCst) as usize;
        let n = 8;

        // Copy from buf[0..] to a temporary, then copy to buf[off..]
        let src: Vec<i32> = buf[0..n as usize].to_vec();
        f(n, &mut buf[off..], &src);

        if !check_range_eq(&buf[off..], &buf[0..], n) {
            return 5;
        }
    }

    0
}