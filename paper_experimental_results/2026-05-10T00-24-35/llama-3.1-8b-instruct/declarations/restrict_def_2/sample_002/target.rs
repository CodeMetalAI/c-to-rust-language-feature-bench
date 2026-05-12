const G_OFF: i32 = 9;

fn f(n: i32, p: &mut [i32], q: &mut [i32]) {
    let mut i = n as usize;
    while i > 0 {
        *p.get_mut(i - 1) = *q.get(i - 1);
        i -= 1;
    }
}

fn check_range_eq(a: &[i32], b: &[i32], n: i32) -> bool {
    for i in 0..n as usize {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut buf = vec![0; 20];
    let mut snapshot = buf.clone();

    for i in 0..20 {
        buf[i] = i * 11 + 3;
        snapshot[i] = buf[i];
    }

    let off = G_OFF;
    let n = 8;

    let p = &mut buf;
    let q = &mut buf[off as usize];

    f(n, p, q);

    if !check_range_eq(&buf[0..n as usize], &snapshot[off as usize..(off + n) as usize], n) {
        panic!("check_range_eq failed");
    }

    if !check_range_eq(&buf[off as usize..(off + n) as usize], &snapshot[off as usize..(off + n) as usize], n) {
        panic!("check_range_eq failed");
    }

    if buf[(off - 1) as usize] != snapshot[(off - 1) as usize] {
        panic!("buffer access failed");
    }

    if buf[(off + n) as usize] != snapshot[(off + n) as usize] {
        panic!("buffer access failed");
    }

    let mut off = G_OFF;
    let n = 8;

    let mut p = &mut buf;
    let mut q = &mut buf[0];

    f(n, p, q);

    if !check_range_eq(&buf[off as usize..(off + n) as usize], &buf[0..n as usize], n) {
        panic!("check_range_eq failed");
    }
}