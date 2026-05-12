static mut g_off: i32 = 9;

fn f(n: i32, p: &mut [i32], q: &mut [i32]) {
    for i in (0..n).rev() {
        p[i as usize] = q[i as usize];
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
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = g_off;
        let n = 8;

        let mut p = &mut buf[0];
        let mut q = &mut buf[off as usize];

        f(n, &mut buf[0..n], &mut buf[off as usize..(off + n) as usize]);

        if !check_range_eq(&buf[0..n], &snapshot[off as usize..(off + n) as usize]) {
            return 1;
        }

        if !check_range_eq(&buf[off as usize..(off + n) as usize], &snapshot[off as usize..(off + n) as usize]) {
            return 2;
        }

        if buf[off as usize - 1] != snapshot[off as usize - 1] {
            return 3;
        }

        if buf[off as usize + n as usize] != snapshot[off as usize + n as usize] {
            return 4;
        }
    }

    {
        let off = g_off;
        let n = 8;

        let mut p = &mut buf[off as usize];
        let mut q = &mut buf[0];

        f(n, &mut buf[off as usize..], &mut buf[..n]);

        if !check_range_eq(&buf[off as usize..], &buf[..n]) {
            return 5;
        }
    }

    0
}