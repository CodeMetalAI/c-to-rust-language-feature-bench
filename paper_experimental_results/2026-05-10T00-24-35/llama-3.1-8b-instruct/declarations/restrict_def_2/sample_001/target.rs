static mut G_OFF: i32 = 9;

fn f(n: usize, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        *p.get_mut(i).unwrap() = *q.get(i).unwrap();
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    for i in 0..n {
        if a[i]!= b[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut buf: [i32; 20] = [0; 20];
    let mut snapshot: [i32; 20] = [0; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = G_OFF;
        let n = 8;

        let mut p = buf.as_mut_slice();
        let mut q = &mut buf[off..];

        f(n, p, q);

        if!check_range_eq(&buf[0..n], &snapshot[off..n], n) {
            panic!("range 1");
        }

        if!check_range_eq(&buf[off..n], &snapshot[off..n], n) {
            panic!("range 2");
        }

        if buf[off - 1]!= snapshot[off - 1] {
            panic!("off - 1");
        }

        if buf[off + n]!= snapshot[off + n] {
            panic!("off + n");
        }
    }

    {
        let off = G_OFF;
        let n = 8;

        let mut p = &mut buf[off..];
        let mut q = &mut buf[..];

        f(n, p, q);

        if!check_range_eq(&buf[off..n], &buf[..n], n) {
            panic!("range 3");
        }
    }
}