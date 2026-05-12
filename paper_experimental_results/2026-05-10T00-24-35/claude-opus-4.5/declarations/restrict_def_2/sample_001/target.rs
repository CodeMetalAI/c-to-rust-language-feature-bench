use std::cell::Cell;

thread_local! {
    static G_OFF: Cell<i32> = Cell::new(9);
}

fn f(mut n: i32, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    while n > 0 {
        n -= 1;
        p[i] = q[i];
        i += 1;
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
    let mut buf = [0i32; 20];
    let mut snapshot = [0i32; 20];

    let mut i = 0;
    while i < 20 {
        buf[i] = (i as i32) * 11 + 3;
        snapshot[i] = buf[i];
        i += 1;
    }

    {
        let off = G_OFF.with(|g| g.get()) as usize;
        let n = 8;

        // Copy from buf[off..] to buf[0..]
        let q_data: Vec<i32> = buf[off..off + n].to_vec();
        f(n as i32, &mut buf[0..n], &q_data);

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
        let off = G_OFF.with(|g| g.get()) as usize;
        let n = 8;

        // Copy from buf[0..] to buf[off..]
        let q_data: Vec<i32> = buf[0..n].to_vec();
        f(n as i32, &mut buf[off..off + n], &q_data);

        if !check_range_eq(&buf[off..off + n], &buf[0..n], n as i32) {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}