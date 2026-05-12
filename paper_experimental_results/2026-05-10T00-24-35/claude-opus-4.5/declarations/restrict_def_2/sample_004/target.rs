static mut G_OFF: i32 = 9;

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
        let off = unsafe { G_OFF } as usize;
        let n = 8;

        // Copy from buf[off..] to buf[0..]
        // We need to copy the source values first since we're working with the same array
        let src_values: Vec<i32> = buf[off..off + n].to_vec();
        for j in 0..n {
            buf[j] = src_values[j];
        }

        if !check_range_eq(&buf[0..], &snapshot[off..], n as i32) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n as i32) {
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
        let off = unsafe { G_OFF } as usize;
        let n = 8;

        // Copy from buf[0..] to buf[off..]
        let src_values: Vec<i32> = buf[0..n].to_vec();
        for j in 0..n {
            buf[off + j] = src_values[j];
        }

        if !check_range_eq(&buf[off..], &buf[0..], n as i32) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}