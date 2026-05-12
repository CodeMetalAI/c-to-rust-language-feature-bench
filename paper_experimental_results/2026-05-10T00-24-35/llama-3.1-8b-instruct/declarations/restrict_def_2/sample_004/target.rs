static mut G_OFF: i32 = 9;

fn f(n: i32, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: i32) -> bool {
    for i in 0..n {
        if a[i as usize] != b[i as usize] {
            return false;
        }
    }
    true
}

fn main() {
    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        let (p, q) = buf.split_at_mut(off);

        f(n, p, q);

        if !check_range_eq(&buf[0..n as usize], &snapshot[off..(off + n) as usize]) {
            panic!("check_range_eq(1) failed");
        }

        if !check_range_eq(&buf[off..], &snapshot[off..n as usize]) {
            panic!("check_range_eq(2) failed");
        }

        if buf[off - 1] != snapshot[off - 1] {
            panic!("buf[off - 1] != snapshot[off - 1]");
        }

        if buf[off + n as usize] != snapshot[off + n as usize] {
            panic!("buf[off + n] != snapshot[off + n]");
        }
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        let (p, q) = buf.split_at_mut(off);

        f(n, p, q);

        if !check_range_eq(&buf[off..n as usize], &buf[0..n as usize]) {
            panic!("check_range_eq(5) failed");
        }
    }

    ()
}