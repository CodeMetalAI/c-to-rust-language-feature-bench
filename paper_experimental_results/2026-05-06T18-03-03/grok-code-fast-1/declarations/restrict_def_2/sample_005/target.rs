const G_OFF: usize = 9;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    for i in 0..n {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut buf: [i32; 20] = [0; 20];
    let mut snapshot: [i32; 20] = [0; 20];

    for i in 0..20 {
        buf[i] = (i as i32) * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = G_OFF;
        let n = 8;

        let (p_slice, _) = buf.split_at_mut(off);
        let p_slice = &mut p_slice[0..n];
        let q_slice = &buf[off..off + n];

        f(n, p_slice, q_slice);

        if !check_range_eq(&buf[0..n], &snapshot[off..off + n], n) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..off + n], &snapshot[off..off + n], n) {
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
        let off = G_OFF;
        let n = 8;

        let (_, p_slice) = buf.split_at_mut(off);
        let p_slice = &mut p_slice[0..n];
        let q_slice = &buf[0..n];

        f(n, p_slice, q_slice);

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}