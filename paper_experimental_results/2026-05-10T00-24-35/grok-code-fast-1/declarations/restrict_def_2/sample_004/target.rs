const G_OFF: i32 = 9;

fn f(n: usize, p: &mut [i32], q: &[i32]) {
    p[0..n].copy_from_slice(&q[0..n]);
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
    a[0..n] == b[0..n]
}

fn main() {
    use std::process;

    let mut buf = [0i32; 20];
    let mut snapshot = [0i32; 20];

    for i in 0..20 {
        buf[i] = i as i32 * 11 + 3;
        snapshot[i] = buf[i];
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        f(n, &mut buf[0..], &buf[off..]);

        if !check_range_eq(&buf[0..], &snapshot[off..], n) {
            process::exit(1);
        }

        if !check_range_eq(&buf[off..], &snapshot[off..], n) {
            process::exit(2);
        }

        if buf[off - 1] != snapshot[off - 1] {
            process::exit(3);
        }

        if buf[off + n] != snapshot[off + n] {
            process::exit(4);
        }
    }

    {
        let off = G_OFF as usize;
        let n = 8;

        f(n, &mut buf[off..], &buf[0..]);

        if !check_range_eq(&buf[off..], &buf[0..], n) {
            process::exit(5);
        }
    }
}