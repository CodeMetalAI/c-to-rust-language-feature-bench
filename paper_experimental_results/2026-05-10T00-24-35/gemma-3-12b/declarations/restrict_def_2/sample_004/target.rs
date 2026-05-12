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
            p = unsafe { &mut *p.add(1) };
            q = unsafe { &mut *q.add(1) };
            j += 1;
        }

        if !check_range_eq(&buf, &snapshot, n, off) {
            return 1;
        }

        if !check_range_eq(&buf, &snapshot, n, off) {
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
            p = unsafe { &mut *p.add(1) };
            q = unsafe { &mut *q.add(1) };
            j += 1;
        }

        if !check_range_eq(&buf, &buf, n, off) {
            return 5;
        }
    }

    0
}

fn check_range_eq(arr: &[i32], snapshot: &[i32], n: usize, off: usize) -> bool {
    let mut i = 0;
    while i < n {
        if arr[off + i] != snapshot[off + i] {
            return false;
        }
        i += 1;
    }
    true
}