fn main() {
    let mut buf: [i32; 20] = [0; 20];
    let mut snapshot: [i32; 20] = [0; 20];

    let mut i = 0;
    while i < 20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = 9;
        let n = 8;

        let mut p = &mut buf[0];
        let mut q = &buf[off];

        let mut k = 0;
        while k < n {
            *p = *q;
            p = &mut *p.add(1);
            q = &*q.add(1);
            k += 1;
        }

        if !check_range_eq(&buf, &snapshot, off, n) {
            return 1;
        }

        if !check_range_eq(&buf, &snapshot, off, n) {
            return 2;
        }

        if buf[off - 1] != snapshot[off - 1] {
            return 3;
        }

        if off + n < 20 && buf[off + n] != snapshot[off + n] {
            return 4;
        }
    }

    {
        let off = 9;
        let n = 8;

        let mut p = &mut buf[off];
        let mut q = &buf[0];

        let mut k = 0;
        while k < n {
            *p = *q;
            p = &mut *p.add(1);
            q = &*q.add(1);
            k += 1;
        }

        if !check_range_eq(&buf, &buf, off, n) {
            return 5;
        }
    }

    0
}

fn check_range_eq(a: &[i32], b: &[i32], off: usize, n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if a[off + i] != b[off + i] {
            return false;
        }
        i += 1;
    }
    true
}