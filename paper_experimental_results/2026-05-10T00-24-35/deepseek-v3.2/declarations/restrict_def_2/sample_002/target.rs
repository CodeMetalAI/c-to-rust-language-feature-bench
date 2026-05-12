fn f(n: usize, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    let mut n = n;
    while n > 0 {
        p[i] = q[i];
        i += 1;
        n -= 1;
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    let mut i = 0;
    while i < n {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    static G_OFF: i32 = 9;
    
    let mut buf = [0i32; 20];
    let mut snapshot = [0i32; 20];

    let mut i = 0;
    while i < 20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        let p_slice = &mut buf[0..];
        let q_slice = &buf[off..];

        f(n, p_slice, q_slice);

        if !check_range_eq(&buf[0..], &snapshot[off..], n) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
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
        let off = G_OFF as usize;
        let n = 8;

        let p_slice = &mut buf[off..];
        let q_slice = &buf[0..];

        f(n, p_slice, q_slice);

        if !check_range_eq(&buf[off..], &buf[0..], n) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}