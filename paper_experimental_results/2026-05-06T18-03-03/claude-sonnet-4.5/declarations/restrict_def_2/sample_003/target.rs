static mut G_OFF: i32 = 9;

fn f(n: i32, p: &mut [i32], q: &[i32]) {
    let mut count = n;
    let mut i = 0;
    while count > 0 {
        p[i] = q[i];
        i += 1;
        count -= 1;
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

        let (p_slice, q_slice) = buf.split_at_mut(off);
        f(n, p_slice, &q_slice[..n as usize]);

        if !check_range_eq(&buf[0..], &snapshot[off..], n) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
            std::process::exit(2);
        }

        if buf[off - 1] != snapshot[off - 1] {
            std::process::exit(3);
        }

        if buf[off + n as usize] != snapshot[off + n as usize] {
            std::process::exit(4);
        }
    }

    {
        let off = unsafe { G_OFF } as usize;
        let n = 8;

        let (q_slice, p_slice) = buf.split_at_mut(off);
        f(n, p_slice, &q_slice[..n as usize]);

        if !check_range_eq(&buf[off..], &buf[0..], n) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}