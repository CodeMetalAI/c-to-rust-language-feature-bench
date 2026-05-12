static mut G_OFF: i32 = 9;

fn f(n: i32, p: &mut i32, q: &mut i32) {
    while n > 0 {
        *p = *q;
        p = p.offset(1);
        q = q.offset(1);
        n -= 1;
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
    let mut buf = [0; 20];
    let mut snapshot = [0; 20];

    for i in 0..20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = G_OFF;
        let n = 8;

        let mut p = buf.as_mut_ptr().add(0);
        let mut q = buf.as_mut_ptr().add(off as usize);

        f(n, p, q);

        if!check_range_eq(&buf[0..n], &snapshot[off as usize..(off as usize + n)], n) {
            panic!("check_range_eq failed");
        }

        if!check_range_eq(&buf[off as usize..(off as usize + n)], &snapshot[off as usize..(off as usize + n)], n) {
            panic!("check_range_eq failed");
        }

        if buf[(off - 1) as usize]!= snapshot[(off - 1) as usize] {
            panic!("buf[off - 1]!= snapshot[off - 1]");
        }

        if buf[(off + n) as usize]!= snapshot[(off + n) as usize] {
            panic!("buf[off + n]!= snapshot[off + n]");
        }
    }

    {
        let off = G_OFF;
        let n = 8;

        let mut p = buf.as_mut_ptr().add(off as usize);
        let mut q = buf.as_mut_ptr().add(0);

        f(n, p, q);

        if!check_range_eq(&buf[off as usize..(off as usize + n)], &buf[0..n], n) {
            panic!("check_range_eq failed");
        }
    }
}