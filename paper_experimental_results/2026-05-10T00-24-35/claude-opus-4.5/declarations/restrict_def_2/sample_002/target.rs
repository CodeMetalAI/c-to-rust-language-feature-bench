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
        let src: Vec<i32> = buf[off..off + n].to_vec();
        f(n as i32, &mut buf[0..n], &src);

        if !check_range_eq(&buf[0..n], &snapshot[off..off + n], n as i32) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..off + n], &snapshot[off..off + n], n as i32) {
            std::process::exit(2);
        }

        if buf[off - 1] != snapshot[off - 1] {
            std::process::exit(3);
        }

        if buf[off + n] != snapshot[off + n] {
            std::process::exit(4);
        }
    }

    {
        let off = G_OFF.load(Ordering::SeqCst) as usize;
        let n = 8;

        // Copy from buf[0..] to a temporary, then copy to buf[off..]
        let src: Vec<i32> = buf[0..n].to_vec();
        f(n as i32, &mut buf[off..off + n], &src);

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n as i32) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}