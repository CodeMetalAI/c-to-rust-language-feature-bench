static mut G_OFF: i32 = 9;

fn f(mut n: i32, p: &mut [i32], q: &[i32]) {
    let mut pi = 0;
    let mut qi = 0;
    while n > 0 {
        n -= 1;
        p[pi] = q[qi];
        pi += 1;
        qi += 1;
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: i32) -> bool {
    let mut i = 0;
    while i < n as usize {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let mut buf: [i32; 20] = [0; 20];
    let mut snapshot: [i32; 20] = [0; 20];

    let mut i = 0;
    while i < 20 {
        buf[i] = (i as i32) * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = unsafe { std::ptr::read_volatile(&G_OFF) } as usize;
        let n = 8;

        // Copy from buf[off..] to buf[0..]
        // We need to copy the source slice first since we can't have mutable and immutable refs
        let src: Vec<i32> = buf[off..off + n].to_vec();
        for j in 0..n {
            buf[j] = src[j];
        }

        if !check_range_eq(&buf[0..n], &snapshot[off..off + n], n as i32) {
            std::process::exit(1);
        }

        if !check_range_eq(&buf[off..off + n], &snapshot[off..off + n], n as i32) {
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
        let off = unsafe { std::ptr::read_volatile(&G_OFF) } as usize;
        let n = 8;

        // Copy from buf[0..] to buf[off..]
        let src: Vec<i32> = buf[0..n].to_vec();
        for j in 0..n {
            buf[off + j] = src[j];
        }

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n as i32) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}