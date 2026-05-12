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
        let mut q = &mut buf[off];

        let mut j = 0;
        while j < n {
            *p = *q;
            p = unsafe { p.add(1) };
            q = unsafe { q.add(1) };
            j += 1;
        }

        if !check_range_eq(&buf, &snapshot, off, n) {
            return 1;
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
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
        let mut q = &mut buf[0];

        let mut j = 0;
        while j < n {
            *p = *q;
            p = unsafe { p.add(1) };
            q = unsafe { q.add(1) };
            j += 1;
        }

        if !check_range_eq(&buf[off..], &buf[0..], n) {
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