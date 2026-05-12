fn main() {
    let mut gate4 = 1;

    fn copy_restrict(n: usize, p: &mut [i32], q: &[i32]) {
        let mut p = p.iter_mut();
        let mut q = q.iter();
        for _ in 0..n {
            *p.next().unwrap() = *q.next().unwrap();
        }
    }

    fn fill(p: &mut [i32], n: usize, base: i32) {
        for (i, elem) in p.iter_mut().enumerate().take(n) {
            *elem = base + i as i32 * 5 + 1;
        }
    }

    fn same(a: &[i32], b: &[i32], n: usize) -> bool {
        a.iter()
           .zip(b.iter())
           .take(n)
           .all(|(a, b)| a == b)
    }

    fn sum(a: &[i32], n: usize) -> i32 {
        a.iter().take(n).sum()
    }

    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    y_snapshot.copy_from_slice(&y);

    {
        let mut p1 = &mut x[..];
        let mut q1 = &y[..];

        if gate4!= 0 {
            let mut p2 = &mut p1[8..];
            let mut q2 = &q1[4..];

            copy_restrict(16, p2, q2);

            let expect: Vec<i32> = y_snapshot[4..20].to_vec();
            if!same(&x[8..24], &expect, 16) {
                return;
            }

            if!same(&y, &y_snapshot, 32) {
                return;
            }
        }

        copy_restrict(8, &mut p1[..8], &q1[24..32]);

        let expect2: Vec<i32> = y_snapshot[24..32].to_vec();
        if!same(&x[..8], &expect2, 8) {
            return;
        }
    }

    if!same(&y, &y_snapshot, 32) {
        return;
    }

    if sum(&x, 32) == 0 {
        return;
    }

    println!("Success");
}