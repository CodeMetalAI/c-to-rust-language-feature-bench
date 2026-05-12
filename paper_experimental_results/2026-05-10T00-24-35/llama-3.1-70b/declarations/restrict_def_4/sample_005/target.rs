fn main() {
    let mut gate4: i32 = 1;

    fn copy_restrict(n: i32, p: &mut [i32], q: &[i32]) {
        for i in 0..n {
            p[i as usize] = q[i as usize];
        }
    }

    fn fill(p: &mut [i32], n: i32, base: i32) {
        for i in 0..n {
            p[i as usize] = base + i * 5 + 1;
        }
    }

    fn same(a: &[i32], b: &[i32], n: i32) -> bool {
        for i in 0..n {
            if a[i as usize]!= b[i as usize] {
                return false;
            }
        }
        true
    }

    fn sum(a: &[i32], n: i32) -> i32 {
        let mut s: i32 = 0;
        for i in 0..n {
            s += a[i as usize];
        }
        s
    }

    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    {
        let mut p1 = &mut x[..];
        let mut q1 = &mut y[..];

        if gate4!= 0 {
            let mut p2 = &mut p1[8..];
            let mut q2 = &mut q1[4..];

            copy_restrict(16, &mut p2[..16], &q2[..16]);

            let expect: [i32; 16] = [
                y_snapshot[4], y_snapshot[5], y_snapshot[6], y_snapshot[7],
                y_snapshot[8], y_snapshot[9], y_snapshot[10], y_snapshot[11],
                y_snapshot[12], y_snapshot[13], y_snapshot[14], y_snapshot[15],
                y_snapshot[16], y_snapshot[17], y_snapshot[18], y_snapshot[19],
            ];

            if!same(&x[8..24], &expect, 16) {
                std::process::exit(1);
            }

            if!same(&y, &y_snapshot, 32) {
                std::process::exit(2);
            }
        }

        copy_restrict(8, &mut p1[..8], &q1[24..32]);

        let expect2: [i32; 8] = [
            y_snapshot[24], y_snapshot[25], y_snapshot[26], y_snapshot[27],
            y_snapshot[28], y_snapshot[29], y_snapshot[30], y_snapshot[31],
        ];

        if!same(&x[..8], &expect2, 8) {
            std::process::exit(3);
        }
    }

    if!same(&y, &y_snapshot, 32) {
        std::process::exit(4);
    }

    if sum(&x, 32) == 0 {
        std::process::exit(5);
    }
}